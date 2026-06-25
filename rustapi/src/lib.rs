pub mod docs;
pub mod extractors;
pub mod routing;

pub use axum::{
    self,
    extract::{Extension, Path, State},
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
pub use routing::{Route, RouterBuilder, RustAPI};

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
pub async fn serve(router: axum::Router<()>) {
    let host = std::env::var("RUSTAPI_HOST").unwrap_or_else(|_| "127.0.0.1".into());
    let port = std::env::var("RUSTAPI_PORT").unwrap_or_else(|_| "3000".into());
    let addr = format!("{}:{}", host, port);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap_or_else(|e| {
            eprintln!("error: could not bind to {}: {}", addr, e);
            std::process::exit(1);
        });

    println!("🚀  Listening on  http://{}", addr);
    println!("📄  Swagger UI    http://{}/docs", addr);
    println!("📘  ReDoc         http://{}/redoc", addr);
    println!("🔧  OpenAPI JSON  http://{}/openapi.json", addr);

    axum::serve(listener, router.into_make_service())
        .await
        .unwrap();
}

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

pub fn schema_for<T: schemars::JsonSchema>() -> serde_json::Value {
    serde_json::to_value(schemars::schema_for!(T)).unwrap()
}

pub struct HTTPException {
    pub status_code: axum::http::StatusCode,
    pub detail: String,
}

impl HTTPException {
    pub fn new(status_code: axum::http::StatusCode, detail: impl Into<String>) -> Self {
        Self {
            status_code,
            detail: detail.into(),
        }
    }
}

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

pub struct Response<T> {
    pub status_code: axum::http::StatusCode,
    pub body: T,
}

impl<T> Response<T> {
    pub fn new(status_code: axum::http::StatusCode, body: T) -> Self {
        Self { status_code, body }
    }
}

impl<T> IntoResponse for Response<T>
where
    T: IntoResponse,
{
    fn into_response(self) -> axum::response::Response {
        (self.status_code, self.body).into_response()
    }
}
