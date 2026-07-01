use crate::http::StatusCode;
pub use axum::response::{Html, IntoResponse, Response as AxumResponse};
pub use axum::Json;
use serde_json::json;

/// A structured exception that can be converted into an HTTP response.
#[derive(Debug)]
pub struct HTTPException {
    pub status_code: StatusCode,
    pub detail: String,
}

impl HTTPException {
    /// Create a new `HTTPException`.
    pub fn new(status_code: StatusCode, detail: impl Into<String>) -> Self {
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
    fn into_response(self) -> AxumResponse {
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
    pub status_code: StatusCode,
    pub body: T,
}

impl<T> Response<T> {
    /// Create a new `Response` with the given status code and body.
    pub fn new(status_code: StatusCode, body: T) -> Self {
        Self { status_code, body }
    }
}

impl<T> IntoResponse for Response<T>
where
    T: IntoResponse,
    T: Send + Sync + 'static,
{
    fn into_response(self) -> AxumResponse {
        (self.status_code, self.body).into_response()
    }
}
