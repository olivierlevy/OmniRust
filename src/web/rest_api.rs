// src/web/rest_api.rs

use axum::{
    routing::{get, post},
    http::StatusCode,
    response::IntoResponse,
    Json, Router, Server,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio::signal; // For graceful shutdown

// Example User struct for request/response
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    id: u64,
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
async fn create_user(Json(payload): Json<User>) -> impl IntoResponse {
    let mut users = USERS_DB.lock().unwrap();
    // Simple ID generation for example purposes
    let new_id = users.len() as u64 + 1;
    let user = User {
        id: new_id,
        username: payload.username,
    };
    users.push(user.clone());
    (StatusCode::CREATED, Json(user))
}

/// Handler for GET /hello
async fn hello_world() -> &'static str {
    "Hello, OmniRust REST API!"
}

/// Configures and returns the Axum router.
pub fn app_router() -> Router {
    Router::new()
        .route("/hello", get(hello_world))
        .route("/users", get(get_users).post(create_user))
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
    
    omnirust::core::init_logger::log_info(&format!("REST API server listening on {}", addr));

    let router = app_router();

    Server::bind(&addr)
        .serve(router.into_make_service())
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
        _ = ctrl_c => {omnirust::core::init_logger::log_info("Received Ctrl+C, shutting down REST server...");},
        _ = terminate => {omnirust::core::init_logger::log_info("Received terminate signal, shutting down REST server...");},
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
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
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        assert_eq!(&body[..], b"Hello, OmniRust REST API!");
    }

    #[tokio::test]
    async fn test_create_and_get_users() {
        // Clear the DB for a clean test state
        USERS_DB.lock().unwrap().clear();

        let app = app_router();

        // Create a user
        let new_user_payload = User { id: 0, username: "testuser".to_string() }; // ID is ignored by create_user
        let response_create = app
            .clone() // Clone router for multiple requests
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/users")
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_string(&new_user_payload).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response_create.status(), StatusCode::CREATED);
        let body_create = hyper::body::to_bytes(response_create.into_body()).await.unwrap();
        let created_user: User = serde_json::from_slice(&body_create).unwrap();
        assert_eq!(created_user.username, "testuser");
        assert_ne!(created_user.id, 0); // ID should be assigned by server

        // Get users
        let response_get = app
            .oneshot(Request::builder().uri("/users").body(Body::empty()).unwrap())
            .await
            .unwrap();
        
        assert_eq!(response_get.status(), StatusCode::OK);
        let body_get = hyper::body::to_bytes(response_get.into_body()).await.unwrap();
        let users: Vec<User> = serde_json::from_slice(&body_get).unwrap();
        
        assert_eq!(users.len(), 1);
        assert_eq!(users[0].username, "testuser");
        assert_eq!(users[0].id, created_user.id);
    }
}
