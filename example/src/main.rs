use axum::middleware::Next;
use axum::response::IntoResponse;
use example::routes::*;
use example::AppState;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use ukiapi::routing::middleware::layers::CorsLayer;
use ukiapi::routing::middleware::MiddlewareExt;
use ukiapi::Request;

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

#[ukiapi::main]
async fn main() {
    let state = AppState {
        items: Arc::new(Mutex::new(Vec::new())),
    };

    ukiapi::routes![AppState, items_router(), auth_router(),]
        .autodiscover()
        .title("Example API")
        .version("1.0.0")
        .on_startup(|_state| async {
            println!("🚀 Application starting up...");
        })
        .on_shutdown(|_state| async {
            println!("🛑 Application shutting down...");
        })
        .mount("/static", ".")
        .middleware(logging_middleware)
        .logger()
        .compression()
        .timeout(Duration::from_secs(30))
        .body_limit(1024 * 1024) // 1MB
        .cors(
            CorsLayer::new()
                .allow_origin(ukiapi::http::HeaderValue::from_static("http://localhost:3000"))
                .allow_methods(vec![ukiapi::http::Method::GET, ukiapi::http::Method::POST])
                .allow_headers(vec![
                    ukiapi::http::header::CONTENT_TYPE,
                    ukiapi::http::header::AUTHORIZATION,
                ]),
        )
        .serve(state)
        .await;
}
