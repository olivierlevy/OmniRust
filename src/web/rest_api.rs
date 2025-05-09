// src/web/rest_api.rs

use axum::{
    routing::get,
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
    middleware, // Added for middleware
};
use serde::{Deserialize, Serialize};
use validator::Validate; // Added for input validation
use crate::web::auth::token_auth_middleware; // Import the auth middleware
use std::net::SocketAddr;
use tokio::net::TcpListener; // Added for Axum 0.7 server
use tokio::signal; // For graceful shutdown

// Example User struct for response and "database" storage
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    id: u64,
    username: String,
}

/// Payload for creating a new user, with validation rules.
#[derive(Deserialize, Debug, Validate)]
struct CreateUserPayload {
    #[validate(length(min = 1, message = "Username cannot be empty"))]
    username: String,
}

// Example in-memory "database" for users
// In a real app, this would interact with a database module.
static USERS_DB: once_cell::sync::Lazy<std::sync::Mutex<Vec<User>>> =
    once_cell::sync::Lazy::new(|| std::sync::Mutex::new(vec![]));

/// Handler for GET /users
async fn get_users() -> impl IntoResponse {
    let users = USERS_DB.lock().unwrap().clone();
    (StatusCode::OK, Json(users))
}

/// Handler for POST /users
async fn create_user(Json(payload): Json<CreateUserPayload>) -> Result<impl IntoResponse, (StatusCode, String)> {
    // Validate the payload
    if let Err(validation_errors) = payload.validate() {
        // Construct a user-friendly error message
        // In a real app, you might format this more nicely or return structured errors.
        let error_messages = validation_errors
            .field_errors()
            .iter()
            .map(|(field, errors)| {
                let messages = errors
                    .iter()
                    .map(|e| e.message.as_ref().map(|s| s.to_string()).unwrap_or_else(|| e.code.to_string()))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{}: {}", field, messages)
            })
            .collect::<Vec<_>>()
            .join("; ");
        return Err((StatusCode::BAD_REQUEST, format!("Input validation failed: {}", error_messages)));
    }

    let mut users = USERS_DB.lock().unwrap();
    // Simple ID generation for example purposes
    let new_id = users.len() as u64 + 1;
    let user = User {
        id: new_id,
        username: payload.username, // Use validated username
    };
    users.push(user.clone());
    Ok((StatusCode::CREATED, Json(user)))
}

/// Handler for GET /hello
async fn hello_world() -> &'static str {
    "Hello, OmniRust REST API!"
}

/// Handler for the protected route
async fn protected_route_handler() -> (StatusCode, &'static str) {
    (StatusCode::OK, "This is a protected route. Authentication successful!")
}

/// Configures and returns the Axum router.
pub fn app_router() -> Router {
    // Define protected routes separately and apply middleware only to them
    let protected_routes = Router::new()
        .route("/protected", get(protected_route_handler))
        .route_layer(middleware::from_fn(token_auth_middleware));

    // Define public routes
    Router::new()
        .route("/hello", get(hello_world))
        .route("/users", get(get_users).post(create_user))
        .merge(protected_routes) // Merge the protected routes
    // Add more routes here
}

/// Starts the Axum REST API server.
///
/// # Arguments
/// * `addr` - The socket address to bind the server to (e.g., "127.0.0.1:3000").
///
/// # Errors
/// Returns an error if the server fails to start.
pub async fn start_rest_server(addr_str: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr: SocketAddr = addr_str.parse()?;
    
    tracing::info!("REST API server listening on {}", addr);

    let router = app_router();
    
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, router.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

// Graceful shutdown signal handler
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>(); // On non-Unix, just wait for Ctrl+C

    tokio::select! {
        _ = ctrl_c => {tracing::info!("Received Ctrl+C, shutting down REST server...");},
        _ = terminate => {tracing::info!("Received terminate signal, shutting down REST server...");},
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::{Body, to_bytes}; // Import axum::body::to_bytes
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt; // for `oneshot` and `ready`

    #[tokio::test]
    async fn test_hello_world_route() {
        let app = app_router();

        let response = app
            .oneshot(Request::builder().uri("/hello").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap(); // Use axum::body::to_bytes
        assert_eq!(&body[..], b"Hello, OmniRust REST API!");
    }

    #[tokio::test]
    async fn test_create_and_get_users() {
        // Clear the DB for a clean test state
        USERS_DB.lock().unwrap().clear();

        let app = app_router();

        // Create a user
        let valid_payload = serde_json::json!({ "username": "testuser" });
        let response_create = app
            .clone() // Clone router for multiple requests
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/users")
                    .header("content-type", "application/json")
                    .body(Body::from(valid_payload.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response_create.status(), StatusCode::CREATED);
        let body_create = to_bytes(response_create.into_body(), usize::MAX).await.unwrap(); // Use axum::body::to_bytes
        let created_user: User = serde_json::from_slice(&body_create).unwrap();
        assert_eq!(created_user.username, "testuser");
        assert_ne!(created_user.id, 0); // ID should be assigned by server

        // Get users
        let response_get = app
            .oneshot(Request::builder().uri("/users").body(Body::empty()).unwrap())
            .await
            .unwrap();
        
        assert_eq!(response_get.status(), StatusCode::OK);
        let body_get = to_bytes(response_get.into_body(), usize::MAX).await.unwrap(); // Use axum::body::to_bytes
        let users: Vec<User> = serde_json::from_slice(&body_get).unwrap();
        
        assert_eq!(users.len(), 1);
        assert_eq!(users[0].username, "testuser");
        assert_eq!(users[0].id, created_user.id);
    }

    #[tokio::test]
    async fn test_protected_route_valid_token() {
        let app = app_router();
        let valid_token = crate::web::auth::EXPECTED_AUTH_TOKEN; // Access the token from auth module

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/protected")
                    .header("Authorization", format!("Bearer {}", valid_token))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        assert_eq!(&body[..], b"This is a protected route. Authentication successful!");
    }

    #[tokio::test]
    async fn test_protected_route_invalid_token() {
        let app = app_router();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/protected")
                    .header("Authorization", "Bearer invalid-token")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_protected_route_missing_bearer_prefix() {
        let app = app_router();
        let valid_token = crate::web::auth::EXPECTED_AUTH_TOKEN;

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/protected")
                    .header("Authorization", valid_token) // Missing "Bearer "
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_protected_route_no_auth_header() {
        let app = app_router();

        let response = app
            .oneshot(Request::builder().uri("/protected").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_create_user_invalid_payload() {
        let app = app_router();

        // Test with empty username
        let invalid_payload = serde_json::json!({ "username": "" });
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/users")
                    .header("content-type", "application/json")
                    .body(Body::from(invalid_payload.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let error_message: String = String::from_utf8_lossy(&body).into();
        assert!(error_message.contains("Input validation failed: username: Username cannot be empty"));

        // Test with missing username (should also fail deserialization or validation if username is not Option<String>)
        // Depending on Serde's default behavior for missing fields, this might be a different error.
        // If username were Option<String> and validated with `#[validate(required)]`, this would be more relevant.
        // For now, an empty string covers the current validation rule.
    }
}
