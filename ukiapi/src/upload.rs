use crate::body::Bytes;
use crate::extract::{FromRequest, Multipart, Request};
use crate::http::StatusCode;
use crate::response::HTTPException;
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

/// A representation of an uploaded file.
#[derive(Debug)]
pub struct UploadFile {
    pub filename: Option<String>,
    pub content_type: Option<String>,
    pub content: Bytes,
}

impl UploadFile {
    /// Save the uploaded file to a destination.
    pub async fn save(&self, destination: impl AsRef<Path>) -> std::io::Result<()> {
        let mut file = File::create(destination).await?;
        file.write_all(&self.content).await?;
        Ok(())
    }
}

impl<S> FromRequest<S> for UploadFile
where
    S: Send + Sync,
{
    type Rejection = HTTPException;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let mut multipart = Multipart::from_request(req, state).await.map_err(|e| {
            HTTPException::new(StatusCode::BAD_REQUEST, format!("Multipart error: {}", e))
        })?;

        while let Some(field) = multipart.next_field().await.map_err(|e| {
            HTTPException::new(
                StatusCode::BAD_REQUEST,
                format!("Multipart field error: {}", e),
            )
        })? {
            if field.file_name().is_some() {
                let filename = field.file_name().map(|s| {
                    let s = s.replace('\\', "/");
                    std::path::Path::new(&s)
                        .file_name()
                        .and_then(|n| n.to_str())
                        .map(|s| s.to_string())
                        .unwrap_or_else(|| "unknown.txt".to_string())
                });
                let content_type = field.content_type().map(|s| s.to_string());
                let content = field.bytes().await.map_err(|e| {
                    HTTPException::new(
                        StatusCode::BAD_REQUEST,
                        format!("Failed to read multipart bytes: {}", e),
                    )
                })?;

                return Ok(UploadFile {
                    filename,
                    content_type,
                    content,
                });
            }
        }

        Err(HTTPException::new(
            StatusCode::BAD_REQUEST,
            "No file uploaded",
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::body::Body;
    use crate::extract::Request;
    use axum::http::header::CONTENT_TYPE;

    async fn create_multipart_request(body: &str, boundary: &str) -> Request {
        Request::builder()
            .header(
                CONTENT_TYPE,
                format!("multipart/form-data; boundary={}", boundary),
            )
            .body(Body::from(body.to_string()))
            .unwrap()
    }

    #[tokio::test]
    async fn test_upload_file_success() {
        let body = "--X-BOUNDARY\r\nContent-Disposition: form-data; name=\"file\"; filename=\"test.txt\"\r\nContent-Type: text/plain\r\n\r\nhello\r\n--X-BOUNDARY--\r\n";
        let req = create_multipart_request(body, "X-BOUNDARY").await;
        let upload = UploadFile::from_request(req, &()).await.unwrap();
        assert_eq!(upload.filename.unwrap(), "test.txt");
        assert_eq!(upload.content_type.unwrap(), "text/plain");
        assert_eq!(upload.content, "hello".as_bytes());
    }

    #[tokio::test]
    async fn test_upload_file_no_file() {
        let body = "--X-BOUNDARY\r\nContent-Disposition: form-data; name=\"other\"\r\n\r\nhello\r\n--X-BOUNDARY--\r\n";
        let req = create_multipart_request(body, "X-BOUNDARY").await;
        let err = UploadFile::from_request(req, &()).await.unwrap_err();
        assert_eq!(err.status_code, StatusCode::BAD_REQUEST);
        assert_eq!(err.detail, "No file uploaded");
    }

    #[tokio::test]
    async fn test_upload_file_iterates_fields() {
        let body = "--X-BOUNDARY\r\nContent-Disposition: form-data; name=\"other\"\r\n\r\nignored metadata\r\n--X-BOUNDARY\r\nContent-Disposition: form-data; name=\"file\"; filename=\"test2.txt\"\r\nContent-Type: text/plain\r\n\r\nworld\r\n--X-BOUNDARY--\r\n";
        let req = create_multipart_request(body, "X-BOUNDARY").await;
        let upload = UploadFile::from_request(req, &()).await.unwrap();
        assert_eq!(upload.filename.unwrap(), "test2.txt");
        assert_eq!(upload.content, "world".as_bytes());
    }
}
