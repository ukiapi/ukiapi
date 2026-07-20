use axum::body::to_bytes;
use axum::http::{header, StatusCode};
use axum::response::IntoResponse;
use std::path::PathBuf;
use ukiapi::{FileResponse, HTMLResponse, PlainTextResponse, RedirectResponse, StreamingResponse};

#[test]
fn test_html_response_new() {
    let response = HTMLResponse::new("<h1>Hello</h1>");
    assert_eq!(response.status_code, StatusCode::OK);
    assert_eq!(response.content, "<h1>Hello</h1>");
}

#[test]
fn test_html_response_with_status() {
    let response = HTMLResponse::new("<h1>Hello</h1>").with_status(StatusCode::CREATED);
    assert_eq!(response.status_code, StatusCode::CREATED);
}

#[tokio::test]
async fn test_html_response_content_type() {
    let response = HTMLResponse::new("<h1>Hello</h1>");
    let res = response.into_response();
    assert_eq!(
        res.headers().get(header::CONTENT_TYPE).unwrap(),
        "text/html; charset=utf-8"
    );
}

#[test]
fn test_plaintext_response_new() {
    let response = PlainTextResponse::new("Hello, World!");
    assert_eq!(response.status_code, StatusCode::OK);
    assert_eq!(response.content, "Hello, World!");
}

#[test]
fn test_plaintext_response_with_status() {
    let response = PlainTextResponse::new("Not Found").with_status(StatusCode::NOT_FOUND);
    assert_eq!(response.status_code, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_plaintext_response_content_type() {
    let response = PlainTextResponse::new("Hello");
    let res = response.into_response();
    assert_eq!(
        res.headers().get(header::CONTENT_TYPE).unwrap(),
        "text/plain; charset=utf-8"
    );
}

#[tokio::test]
async fn test_plaintext_response_body() {
    let response = PlainTextResponse::new("Test content");
    let res = response.into_response();
    let body = to_bytes(res.into_body(), usize::MAX).await.unwrap();
    assert_eq!(body, "Test content");
}

#[test]
fn test_redirect_response_temporary() {
    let response = RedirectResponse::temporary("/new-location");
    assert_eq!(response.status_code, StatusCode::TEMPORARY_REDIRECT);
    assert_eq!(response.url, "/new-location");
}

#[test]
fn test_redirect_response_permanent() {
    let response = RedirectResponse::permanent("/permanent-location");
    assert_eq!(response.status_code, StatusCode::PERMANENT_REDIRECT);
    assert_eq!(response.url, "/permanent-location");
}

#[test]
fn test_redirect_response_to() {
    let response = RedirectResponse::to("/see-other");
    assert_eq!(response.status_code, StatusCode::SEE_OTHER);
    assert_eq!(response.url, "/see-other");
}

#[test]
fn test_redirect_response_temporary_into_response() {
    let response = RedirectResponse::temporary("/temp");
    let res = response.into_response();
    assert_eq!(res.status(), StatusCode::TEMPORARY_REDIRECT);
}

#[test]
fn test_redirect_response_permanent_into_response() {
    let response = RedirectResponse::permanent("/perm");
    let res = response.into_response();
    assert_eq!(res.status(), StatusCode::PERMANENT_REDIRECT);
}

#[test]
fn test_file_response_new() {
    let response = FileResponse::new("/path/to/file.txt");
    assert_eq!(response.status_code, StatusCode::OK);
    assert_eq!(response.path, PathBuf::from("/path/to/file.txt"));
    assert!(response.media_type.is_none());
}

#[test]
fn test_file_response_with_media_type() {
    let response = FileResponse::new("/path/to/file.txt").with_media_type("text/plain");
    assert_eq!(response.media_type, Some("text/plain".to_string()));
}

#[test]
fn test_file_response_missing_file() {
    let response = FileResponse::new("/nonexistent/file.txt");
    let res = response.into_response();
    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_streaming_response_new() {
    let stream = futures::stream::iter(vec![
        Ok::<_, std::io::Error>(vec![1, 2, 3]),
        Ok::<_, std::io::Error>(vec![4, 5, 6]),
    ]);
    let response = StreamingResponse::new(stream);
    assert_eq!(response.status_code, StatusCode::OK);
    assert_eq!(response.media_type, "application/octet-stream");
}

#[test]
fn test_streaming_response_with_media_type() {
    let stream = futures::stream::iter(vec![Ok::<_, std::io::Error>(vec![1, 2, 3])]);
    let response = StreamingResponse::new(stream).with_media_type("text/plain");
    assert_eq!(response.media_type, "text/plain");
}

#[test]
fn test_streaming_response_with_status() {
    let stream = futures::stream::iter(vec![Ok::<_, std::io::Error>(vec![1, 2, 3])]);
    let response = StreamingResponse::new(stream).with_status(StatusCode::PARTIAL_CONTENT);
    assert_eq!(response.status_code, StatusCode::PARTIAL_CONTENT);
}
