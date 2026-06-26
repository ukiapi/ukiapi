use rustapi::{post, routes, serve, UploadFile, Json};

#[post("/upload")]
async fn upload_handler(
    file: UploadFile,
) -> Json<serde_json::Value> {
    let filename = file.filename.clone().unwrap_or_else(|| "unknown.txt".to_string());
    let size = file.content.len();

    // Save the file
    if let Err(e) = file.save(&filename).await {
        return rustapi::json!({
            "error": format!("Failed to save file: {}", e)
        })
        .into();
    }

    rustapi::json!({
        "message": "File uploaded successfully",
        "filename": filename,
        "size": size,
        "content_type": file.content_type
    })
    .into()
}

#[tokio::main]
async fn main() {
    let state = ();

    let api = routes![(), upload_handler_route()];

    serve(api.build_router(state)).await;
}
