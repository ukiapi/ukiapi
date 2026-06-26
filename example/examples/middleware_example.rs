use rustapi::{get, Request, RustAPI, MiddlewareExt, middleware::CorsLayer};
use axum::middleware::Next;
use axum::response::IntoResponse;
use std::time::Duration;

#[get("/")]
async fn index() -> &'static str {
    "Hello with middleware!"
}

async fn logging_middleware(req: Request, next: Next) -> axum::response::Response {
    println!("--- Custom Middleware: Processing {} {} ---", req.method(), req.uri());
    let response = next.run(req).await;
    println!("--- Custom Middleware: Response status: {} ---", response.status());
    response.into_response()
}

#[tokio::main]
async fn main() {
    // Standard RustAPI setup with the new MiddlewareExt trait
    RustAPI::<()>::new()
        .route(index_route())
        // 1. Custom function-based middleware
        .middleware(logging_middleware)
        // 2. Built-in helpers for common middleware
        .logger()
        .compression()
        .timeout(Duration::from_secs(30))
        .body_limit(1024 * 1024) // 1MB
        // 3. Tower layers via helper or use_layer
        .cors(CorsLayer::permissive())
        .title("Middleware Example")
        .serve(())
        .await;
}
