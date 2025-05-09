// src/web/auth.rs

use axum::{
    extract::Request,
    http::{HeaderMap, HeaderValue, StatusCode},
    middleware::Next,
    response::Response,
};

// For this basic example, the token is hardcoded.
// In a real application, this would come from config or a secure store,
// and you'd likely use JWTs or a similar mechanism.
const EXPECTED_AUTH_TOKEN: &str = "omnirust-secure-token-123";

/// Middleware for simple bearer token authentication.
pub async fn token_auth_middleware(
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = headers
        .get("Authorization")
        .and_then(|header| header.to_str().ok());

    if let Some(auth_value) = auth_header {
        if let Some(token) = auth_value.strip_prefix("Bearer ") {
            if token == EXPECTED_AUTH_TOKEN {
                // Token is valid, proceed with the request
                return Ok(next.run(request).await);
            }
        }
    }

    // Token is missing, malformed, or invalid
    Err(StatusCode::UNAUTHORIZED)
}

// A more advanced version might extract claims from a JWT token
// and add them to request extensions for use by handlers.
// pub async fn jwt_auth_middleware<B>(
//     mut req: Request<B>,
//     next: Next<B>,
// ) -> Result<Response, StatusCode> {
//     // ... JWT parsing and validation logic ...
//     // if valid:
//     //   req.extensions_mut().insert(user_claims);
//     //   Ok(next.run(req).await)
//     // else:
//     //   Err(StatusCode::UNAUTHORIZED)
// }
