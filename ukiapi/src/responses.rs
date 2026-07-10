use axum::{
    body::Body,
    http::{header, HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};
use std::path::PathBuf;
use tokio_util::io::ReaderStream;

/// A response that returns HTML content.
pub struct HTMLResponse {
    pub content: String,
    pub status_code: StatusCode,
    pub headers: HeaderMap,
}

impl HTMLResponse {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            status_code: StatusCode::OK,
            headers: HeaderMap::new(),
        }
    }

    pub fn with_status(mut self, status: StatusCode) -> Self {
        self.status_code = status;
        self
    }

    pub fn with_header(mut self, key: header::HeaderName, value: header::HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
}

impl IntoResponse for HTMLResponse {
    fn into_response(self) -> Response {
        let mut res = (self.status_code, axum::response::Html(self.content)).into_response();
        let headers = res.headers_mut();
        for (key, value) in self.headers {
            if let Some(key) = key {
                headers.insert(key, value);
            }
        }
        headers.insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("text/html; charset=utf-8"),
        );
        res
    }
}

/// A response that returns plain text content.
pub struct PlainTextResponse {
    pub content: String,
    pub status_code: StatusCode,
    pub headers: HeaderMap,
}

impl PlainTextResponse {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            status_code: StatusCode::OK,
            headers: HeaderMap::new(),
        }
    }

    pub fn with_status(mut self, status: StatusCode) -> Self {
        self.status_code = status;
        self
    }

    pub fn with_header(mut self, key: header::HeaderName, value: header::HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
}

impl IntoResponse for PlainTextResponse {
    fn into_response(self) -> Response {
        let mut res = (self.status_code, self.content).into_response();
        let headers = res.headers_mut();
        for (key, value) in self.headers {
            if let Some(key) = key {
                headers.insert(key, value);
            }
        }
        headers.insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("text/plain; charset=utf-8"),
        );
        res
    }
}

/// A response that redirects to a different URL.
pub struct RedirectResponse {
    pub url: String,
    pub status_code: StatusCode,
}

impl RedirectResponse {
    /// 307 Temporary Redirect
    pub fn temporary(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            status_code: StatusCode::TEMPORARY_REDIRECT,
        }
    }

    /// 308 Permanent Redirect
    pub fn permanent(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            status_code: StatusCode::PERMANENT_REDIRECT,
        }
    }

    /// 303 See Other
    pub fn to(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            status_code: StatusCode::SEE_OTHER,
        }
    }
}

impl IntoResponse for RedirectResponse {
    fn into_response(self) -> Response {
        match self.status_code {
            StatusCode::MOVED_PERMANENTLY => {
                axum::response::Redirect::permanent(&self.url).into_response()
            }
            StatusCode::TEMPORARY_REDIRECT => {
                axum::response::Redirect::temporary(&self.url).into_response()
            }
            StatusCode::PERMANENT_REDIRECT => {
                axum::response::Redirect::permanent(&self.url).into_response()
            }
            _ => axum::response::Redirect::to(&self.url).into_response(),
        }
    }
}

/// A response that serves a file from the disk.
pub struct FileResponse {
    pub path: PathBuf,
    pub status_code: StatusCode,
    pub headers: HeaderMap,
    pub media_type: Option<String>,
}

impl FileResponse {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self {
            path: path.into(),
            status_code: StatusCode::OK,
            headers: HeaderMap::new(),
            media_type: None,
        }
    }

    pub fn with_media_type(mut self, media_type: impl Into<String>) -> Self {
        self.media_type = Some(media_type.into());
        self
    }

    pub fn with_status(mut self, status: StatusCode) -> Self {
        self.status_code = status;
        self
    }

    pub fn with_header(mut self, key: header::HeaderName, value: header::HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
}

impl IntoResponse for FileResponse {
    fn into_response(self) -> Response {
        let file = match std::fs::File::open(&self.path) {
            Ok(file) => file,
            Err(_) => return (StatusCode::NOT_FOUND, "File not found").into_response(),
        };

        let tokio_file = tokio::fs::File::from_std(file);
        let stream = ReaderStream::new(tokio_file);
        let body = Body::from_stream(stream);

        let mut res = (self.status_code, body).into_response();
        let headers = res.headers_mut();
        for (key, value) in self.headers {
            if let Some(key) = key {
                headers.insert(key, value);
            }
        }

        let mime = self.media_type.unwrap_or_else(|| {
            mime_guess::from_path(&self.path)
                .first_raw()
                .unwrap_or("application/octet-stream")
                .to_string()
        });

        headers.insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_str(&mime)
                .unwrap_or(header::HeaderValue::from_static("application/octet-stream")),
        );

        res
    }
}

/// A response that streams data.
pub struct StreamingResponse<S> {
    pub stream: S,
    pub status_code: StatusCode,
    pub headers: HeaderMap,
    pub media_type: String,
}

impl<S, T, E> StreamingResponse<S>
where
    S: futures::Stream<Item = Result<T, E>> + Send + 'static,
    T: Into<axum::body::Bytes> + 'static,
    E: Into<Box<dyn std::error::Error + Send + Sync>> + 'static,
{
    pub fn new(stream: S) -> Self {
        Self {
            stream,
            status_code: StatusCode::OK,
            headers: HeaderMap::new(),
            media_type: "application/octet-stream".to_string(),
        }
    }

    pub fn with_media_type(mut self, media_type: impl Into<String>) -> Self {
        self.media_type = media_type.into();
        self
    }

    pub fn with_status(mut self, status: StatusCode) -> Self {
        self.status_code = status;
        self
    }

    pub fn with_header(mut self, key: header::HeaderName, value: header::HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
}

impl<S, T, E> IntoResponse for StreamingResponse<S>
where
    S: futures::Stream<Item = Result<T, E>> + Send + 'static,
    T: Into<axum::body::Bytes> + 'static,
    E: Into<Box<dyn std::error::Error + Send + Sync>> + 'static,
{
    fn into_response(self) -> Response {
        let body = Body::from_stream(self.stream);
        let mut res = (self.status_code, body).into_response();
        let headers = res.headers_mut();
        for (key, value) in self.headers {
            if let Some(key) = key {
                headers.insert(key, value);
            }
        }
        headers.insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_str(&self.media_type)
                .unwrap_or(header::HeaderValue::from_static("application/octet-stream")),
        );
        res
    }
}
