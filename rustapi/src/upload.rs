use axum::{
    extract::{FromRequest, Multipart},
};
use axum::body::Bytes;
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use crate::HTTPException;

/// A representation of an uploaded file.
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

    async fn from_request(req: axum::extract::Request, state: &S) -> Result<Self, Self::Rejection> {
        let mut multipart = Multipart::from_request(req, state).await
            .map_err(|e| HTTPException::new(axum::http::StatusCode::BAD_REQUEST, format!("Multipart error: {}", e)))?;

        if let Some(field) = multipart.next_field().await
            .map_err(|e| HTTPException::new(axum::http::StatusCode::BAD_REQUEST, format!("Multipart field error: {}", e)))? {

            let filename = field.file_name().map(|s| s.to_string());
            let content_type = field.content_type().map(|s| s.to_string());
            let content = field.bytes().await
                .map_err(|e| HTTPException::new(axum::http::StatusCode::BAD_REQUEST, format!("Failed to read multipart bytes: {}", e)))?;

            return Ok(UploadFile {
                filename,
                content_type,
                content,
            });
        }

        Err(HTTPException::new(axum::http::StatusCode::BAD_REQUEST, "No file uploaded"))
    }
}
