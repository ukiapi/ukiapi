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
        // ⚡ Bolt: Move self.detail to avoid cloning the string allocation on every non-server-error response
        let safe_detail = if self.status_code.is_server_error() {
            "Internal Server Error".to_string()
        } else {
            self.detail
        };

        (
            self.status_code,
            Json(json!({
                "detail": safe_detail,
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

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::to_bytes;

    #[test]
    fn test_http_exception_new() {
        let exc = HTTPException::new(StatusCode::NOT_FOUND, "Resource not found");
        assert_eq!(exc.status_code, StatusCode::NOT_FOUND);
        assert_eq!(exc.detail, "Resource not found");
    }

    #[test]
    fn test_http_exception_display() {
        let exc = HTTPException::new(StatusCode::BAD_REQUEST, "Invalid input");
        let display = format!("{}", exc);
        assert!(display.contains("400"));
        assert!(display.contains("Invalid input"));
    }

    #[test]
    fn test_http_exception_into_response() {
        let exc = HTTPException::new(StatusCode::UNAUTHORIZED, "Unauthorized access");
        let response = exc.into_response();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_http_exception_response_body() {
        let exc = HTTPException::new(StatusCode::FORBIDDEN, "Forbidden resource");
        let response = exc.into_response();
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["detail"], "Forbidden resource");
    }

    #[test]
    fn test_response_new() {
        let response = Response::new(StatusCode::OK, "test body");
        assert_eq!(response.status_code, StatusCode::OK);
        assert_eq!(response.body, "test body");
    }

    #[test]
    fn test_response_into_response() {
        let response = Response::new(StatusCode::CREATED, "created resource");
        let axum_response = response.into_response();
        assert_eq!(axum_response.status(), StatusCode::CREATED);
    }

    #[test]
    fn test_http_exception_is_error() {
        let exc = HTTPException::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error");
        let error: &dyn std::error::Error = &exc;
        assert!(error.source().is_none());
    }

    #[tokio::test]
    async fn test_http_exception_json_structure() {
        let exc = HTTPException::new(StatusCode::BAD_REQUEST, "Validation failed");
        let response = exc.into_response();
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert!(json.is_object());
        assert!(json.get("detail").is_some());
    }
}
