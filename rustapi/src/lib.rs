pub mod auth;
pub mod background_tasks;
pub mod body;
pub mod dependencies;
pub mod docs;
pub mod extract;
pub mod extractors;
pub mod handler;
pub mod http;
pub mod lifecycle;
pub mod middleware;
pub mod mount;
pub mod response;
pub mod responses;
pub mod routing;
pub mod server;
pub mod static_files;
pub mod test_client;
pub mod tower;
pub mod upload;
pub mod utils;

pub use axum;
pub use rustapi_macros::{delete, get, model, patch, post, put};
pub use schemars::JsonSchema;
pub use serde::{Deserialize, Serialize};
pub use serde_json::{json, Value};
pub use ts_rs;
pub use validator::Validate;

pub use auth::{decode_jwt, encode_jwt, HTTPBearer, JWTAuth, OAuth2PasswordBearer};
pub use background_tasks::BackgroundTasks;
pub use dependencies::{security, Dependency, Depends, Security};
pub use env_logger;
pub use extract::{Path, Request, State};
pub use extractors::{Query, ValidatedJson};
pub use log::{error, info};
pub use response::{HTTPException, Json, Response};
pub use responses::{FileResponse, HTMLResponse, PlainTextResponse, RedirectResponse, StreamingResponse};
pub use utils::jsonable_encoder;
pub use routing::{APIRouter, Routable, Route, Router, RouterBuilder, RustAPI};
pub use test_client::TestClient;
pub use upload::UploadFile;

/// Start the server. Reads `RUSTAPI_HOST` and `RUSTAPI_PORT` from the
/// environment (set automatically by `rustapi run` / `rustapi dev`),
/// falling back to `127.0.0.1:3000`.
///
/// ```rust,no_run
/// #[tokio::main]
/// async fn main() {
///     // let app = rustapi::routes![AppState, ...].build_router(state);
///     // rustapi::serve(app).await;
/// }
/// ```
pub async fn serve(router: Router<()>) {
    env_logger::init();
    info!("Initializing RustAPI server...");

    let host = std::env::var("RUSTAPI_HOST").unwrap_or_else(|_| "127.0.0.1".into());
    let port = std::env::var("RUSTAPI_PORT").unwrap_or_else(|_| "3000".into());
    let addr = format!("{}:{}", host, port);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap_or_else(|e| {
            error!("Could not bind to {}: {}", addr, e);
            std::process::exit(1);
        });

    info!("🚀  Listening on  http://{}", addr);
    info!("📄  Swagger UI    http://{}/docs", addr);
    info!("📘  ReDoc         http://{}/redoc", addr);
    info!("🔧  OpenAPI JSON  http://{}/openapi.json", addr);

    crate::server::serve(listener, router.into_make_service())
        .with_graceful_shutdown(async move {
            tokio::signal::ctrl_c()
                .await
                .expect("failed to install CTRL+C handler");
            info!("Received shutdown signal.");
        })
        .await
        .unwrap();
}

/// A macro to initialize a `RustAPI` instance with a set of routes.
///
/// Usage: `routes![AppState, route1(), route2()]`
#[macro_export]
macro_rules! routes {
    ($state:ty, $($x:expr),* $(,)?) => {
        {
            let mut api = $crate::RustAPI::<$state>::new();
            $(
                api = api.route($x);
            )*
            api
        }
    };
}

/// Helper to generate a JSON schema for a type.
pub fn schema_for<T: schemars::JsonSchema>() -> serde_json::Value {
    serde_json::to_value(schemars::schema_for!(T)).unwrap()
}
