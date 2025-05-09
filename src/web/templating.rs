// src/web/templating.rs

use askama::Template; // Main derive macro
use axum::response::Html; // Axum HTML response type
use axum::routing::get;
use axum::Router;

// Define a struct that represents the data for your template.
#[derive(Template)]
#[template(path = "hello.html")] // Path to the template file in the `templates` directory
pub struct HelloTemplate<'a> {
    name: &'a str,
    items: Vec<&'a str>,
}

// Handler that renders the HelloTemplate
async fn render_hello_template() -> Html<String> {
    let template = HelloTemplate {
        name: "OmniRust User",
        items: vec!["Item 1", "Item 2", "Item 3 from Askama"],
    };
    match template.render() {
        Ok(html) => Html(html),
        Err(e) => Html(format!("<html><body>Error rendering template: {}</body></html>", e)),
    }
}

/// Adds templating related routes to an existing Axum router.
pub fn add_template_routes(router: Router) -> Router {
    router.route("/greet", get(render_hello_template))
    // Add more template routes here
}

// If you want to run a standalone server for template examples:
// pub async fn start_template_server(addr_str: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//     let addr: std::net::SocketAddr = addr_str.parse()?;
//     omnirust::core::init_logger::log_info(&format!("Template example server listening on {}", addr));
//     let app = add_template_routes(Router::new());
//     axum::Server::bind(&addr)
//         .serve(app.into_make_service())
//         .await?;
//     Ok(())
// }

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt; // for `oneshot`

    #[tokio::test]
    async fn test_render_hello_template_route() {
        let app = add_template_routes(Router::new());

        let response = app
            .oneshot(Request::builder().uri("/greet").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body_str = String::from_utf8(body.to_vec()).unwrap();

        // Check for key parts of the rendered template
        assert!(body_str.contains("<h1>Hello, OmniRust User!</h1>"));
        assert!(body_str.contains("<li>Item 1</li>"));
        assert!(body_str.contains("<li>Item 2</li>"));
        assert!(body_str.contains("<li>Item 3 from Askama</li>"));
    }
}
