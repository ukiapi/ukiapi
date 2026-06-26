pub mod docs;
pub mod extractors;
pub mod dependencies;
pub mod background_tasks;
pub mod upload;
pub mod test_client;
pub mod routing;
pub mod lifecycle;
pub mod mount;
pub mod responses;
pub mod utils;

pub use axum::{
    self,
    extract::{Extension, Path, Request, State},
    response::{Html, IntoResponse},
    Json,
};
pub use rustapi_macros::{delete, get, model, patch, post, put};
pub use schemars::JsonSchema;
pub use serde::{Deserialize, Serialize};
pub use serde_json::{json, Value};
pub use ts_rs;
pub use validator::Validate;

pub use extractors::{Query, ValidatedJson};
pub use dependencies::{Dependency, Depends, Security, security};
pub use background_tasks::BackgroundTasks;
pub use upload::UploadFile;
pub use test_client::TestClient;
pub use routing::{APIRouter, Routable, RouterBuilder, RustAPI, Route, MiddlewareExt};
pub use routing::middleware::layers as middleware;
pub use log::{info, error};
pub use env_logger;

pub use responses::{
    FileResponse, HTMLResponse, PlainTextResponse, RedirectResponse, StreamingResponse,
};
pub use utils::jsonable_encoder;

/// Start the server. Reads `RUSTAPI_HOST` and `RUSTAPI_PORT` from the
/// environment (set automatically by `rustapi run` / `rustapi dev`),
/// falling back to `127.0.0.1:3000`.
pub async fn serve(router: axum::Router<()>) {
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

    axum::serve(listener, router.into_make_service())
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

/// A structured exception that can be converted into an HTTP response.
#[derive(Debug)]
pub struct HTTPException {
    pub status_code: axum::http::StatusCode,
    pub detail: String,
}

impl HTTPException {
    /// Create a new `HTTPException`.
    pub fn new(status_code: axum::http::StatusCode, detail: impl Into<String>) -> Self {
        Self {
            status_code,
            detail: detail.into(),
        }
    }
}

impl std::fmt::Display for HTTPException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HTTPException {}: {}", self.status_code, self.detail)
    }
}

impl std::error::Error for HTTPException {}

impl IntoResponse for HTTPException {
    fn into_response(self) -> axum::response::Response {
        (
            self.status_code,
            Json(json!({
                "detail": self.detail,
            })),
        )
            .into_response()
    }
}

/// A wrapper for responses to explicitly set the HTTP status code.
pub struct Response<T> {
    pub status_code: axum::http::StatusCode,
    pub body: T,
}

impl<T> Response<T> {
    /// Create a new `Response` with the given status code and body.
    pub fn new(status_code: axum::http::StatusCode, body: T) -> Self {
        Self { status_code, body }
    }
}

impl<T> IntoResponse for Response<T>
where
    T: IntoResponse,
    T: Send + Sync + 'static,
{
    fn into_response(self) -> axum::response::Response {
        (self.status_code, self.body).into_response()
    }
}
