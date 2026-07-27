use axum::body::Body;
use ukiapi::extract::FromRequest;
use ukiapi::extract::Request as UkiRequest;
use ukiapi::http::StatusCode;
use ukiapi::upload::UploadFile;

#[tokio::test]
async fn test_upload_missing_file() {
    let req = UkiRequest::builder()
        .method("POST")
        .header(
            "content-type",
            "multipart/form-data; boundary=X-TEST-BOUNDARY",
        )
        .body(Body::from(
            "--X-TEST-BOUNDARY\r\n\
             Content-Disposition: form-data; name=\"not_a_file\"\r\n\r\n\
             just some text\r\n\
             --X-TEST-BOUNDARY--\r\n",
        ))
        .unwrap();

    let result = UploadFile::from_request(req, &()).await;
    match result {
        Err(err) => {
            assert_eq!(err.status_code, StatusCode::BAD_REQUEST);
            assert_eq!(err.detail, "No file uploaded");
        }
        Ok(_) => panic!("Expected error for missing file"),
    }
}

#[tokio::test]
async fn test_upload_valid_file() {
    let req = UkiRequest::builder()
        .method("POST")
        .header(
            "content-type",
            "multipart/form-data; boundary=X-TEST-BOUNDARY",
        )
        .body(Body::from(
            "--X-TEST-BOUNDARY\r\n\
             Content-Disposition: form-data; name=\"not_a_file\"\r\n\r\n\
             just some text\r\n\
             --X-TEST-BOUNDARY\r\n\
             Content-Disposition: form-data; name=\"file\"; filename=\"test.txt\"\r\n\
             Content-Type: text/plain\r\n\r\n\
             hello world\r\n\
             --X-TEST-BOUNDARY--\r\n",
        ))
        .unwrap();

    let result = UploadFile::from_request(req, &()).await;
    match result {
        Ok(file) => {
            assert_eq!(file.filename, Some("test.txt".to_string()));
            assert_eq!(file.content_type, Some("text/plain".to_string()));
            assert_eq!(file.content, "hello world".as_bytes());
        }
        Err(e) => panic!("Expected success, got {:?}", e),
    }
}
