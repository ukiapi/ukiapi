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

        if let Some(field) = multipart.next_field().await.map_err(|e| {
            HTTPException::new(
                StatusCode::BAD_REQUEST,
                format!("Multipart field error: {}", e),
            )
        })? {
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

        Err(HTTPException::new(
            StatusCode::BAD_REQUEST,
            "No file uploaded",
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::extract::FromRequest;
    use crate::extract::Request;
    use crate::http::StatusCode;
    use axum::body::Body;
    use axum::http::header::CONTENT_TYPE;

    #[tokio::test]
    async fn test_upload_file_from_request_valid() {
        let boundary = "------------------------14737809831466499882746641449";
        let body_content = format!(
            "--{boundary}\r\n\
             Content-Disposition: form-data; name=\"file\"; filename=\"test.txt\"\r\n\
             Content-Type: text/plain\r\n\
             \r\n\
             hello world\r\n\
             --{boundary}--\r\n"
        );
        let req = Request::builder()
            .method("POST")
            .header(
                CONTENT_TYPE,
                format!("multipart/form-data; boundary={}", boundary),
            )
            .body(Body::from(body_content))
            .unwrap();

        let result = UploadFile::from_request(req, &()).await;
        assert!(result.is_ok());
        let upload = result.unwrap();
        assert_eq!(upload.filename.as_deref(), Some("test.txt"));
        assert_eq!(upload.content_type.as_deref(), Some("text/plain"));
        assert_eq!(&upload.content[..], b"hello world");
    }

    #[tokio::test]
    async fn test_upload_file_from_request_missing_boundary() {
        let req = Request::builder()
            .method("POST")
            .header(CONTENT_TYPE, "multipart/form-data")
            .body(Body::from(""))
            .unwrap();

        let result = UploadFile::from_request(req, &()).await;
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.status_code, StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_upload_file_from_request_no_file() {
        let boundary = "------------------------14737809831466499882746641449";
        let body_content = format!("--{boundary}--\r\n");
        let req = Request::builder()
            .method("POST")
            .header(
                CONTENT_TYPE,
                format!("multipart/form-data; boundary={}", boundary),
            )
            .body(Body::from(body_content))
            .unwrap();

        let result = UploadFile::from_request(req, &()).await;
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.status_code, StatusCode::BAD_REQUEST);
        assert_eq!(err.detail, "No file uploaded");
    }
}
