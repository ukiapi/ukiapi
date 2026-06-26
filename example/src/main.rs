use axum::middleware::Next;
use axum::response::IntoResponse;
use example::routes::*;
use example::AppState;
use rustapi::middleware::CorsLayer;
use rustapi::{MiddlewareExt, Request};
use std::sync::{Arc, Mutex};
use std::time::Duration;

async fn logging_middleware(req: Request, next: Next) -> axum::response::Response {
    println!(
        "--- Custom Middleware: Processing {} {} ---",
        req.method(),
        req.uri()
    );
    let response = next.run(req).await;
    println!(
        "--- Custom Middleware: Response status: {} ---",
        response.status()
    );
    response.into_response()
}

#[tokio::main]
async fn main() {
    let state = AppState {
        items: Arc::new(Mutex::new(Vec::new())),
    };

    rustapi::routes![
        AppState,
        hello_route().with_state::<AppState>(),
        items_router(),
        trigger_error_route().with_state::<AppState>(),
        background_handler_route().with_state::<AppState>(),
        upload_handler_route().with_state::<AppState>()
    ]
    .title("Example API")
    .version("1.0.0")
    .on_startup(|_state| async {
        println!("🚀 Application starting up...");
    })
    .on_shutdown(|_state| async {
        println!("🛑 Application shutting down...");
    })
    .mount("/static", ".")
    // Middleware
    .middleware(logging_middleware)
    .logger()
    .compression()
    .timeout(Duration::from_secs(30))
    .body_limit(1024 * 1024) // 1MB
    .cors(CorsLayer::permissive())
    .serve(state)
    .await;
}
