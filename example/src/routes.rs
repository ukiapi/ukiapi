use crate::models::{
    ItemCreate, ItemDb, ItemResponse, ListItemsQuery, LoginRequest, TokenResponse, UserClaims,
};
use crate::AppState;
use rustapi::http::StatusCode;
use rustapi::State;
use rustapi::{
    encode_jwt, error, get, info, jsonable_encoder, post, APIRouter, BackgroundTasks, Depends,
    FileResponse, HTMLResponse, HTTPException, JWTAuth, RedirectResponse, Response, UploadFile,
    ValidatedJson,
};
use std::fs::OpenOptions;
use std::io::Write;

#[get("/hello")]
pub async fn hello() -> &'static str {
    info!("Accessed /hello route.");
    "Hello from RustAPI!"
}

#[post("/login")]
pub async fn login(
    ValidatedJson(body): ValidatedJson<LoginRequest>,
) -> Result<rustapi::Json<TokenResponse>, HTTPException> {
    info!("Logging in user: {}", body.username);

    let expiration = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        + 3600;

    let claims = UserClaims {
        sub: body.username,
        exp: expiration,
    };

    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret".to_string());
    let token = encode_jwt(&claims, &secret).map_err(|e| {
        HTTPException::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to generate token: {}", e),
        )
    })?;

    Ok(rustapi::Json(TokenResponse {
        access_token: token,
        token_type: "Bearer".to_string(),
    }))
}

#[get("/me")]
pub async fn me(Depends(claims, _): Depends<JWTAuth<UserClaims>>) -> rustapi::Json<UserClaims> {
    rustapi::Json(claims)
}

#[get("")]
pub async fn list_items(
    State(state): State<AppState>,
    rustapi::Query(query): rustapi::Query<ListItemsQuery>,
) -> rustapi::Json<rustapi::Value> {
    info!("Accessed /items route with query: {:?}", query.q);
    let items = state.items.lock().unwrap();
    let limit = query.limit.unwrap_or(10) as usize;

    let mut results: Vec<ItemResponse> = items
        .iter()
        .filter(|item| {
            if let Some(ref q) = query.q {
                item.name.contains(q)
            } else {
                true
            }
        })
        .map(|item| ItemResponse {
            id: item.id,
            name: item.name.clone(),
            price: item.price,
        })
        .collect();

    results.truncate(limit);
    // Demonstrate jsonable_encoder
    rustapi::Json(jsonable_encoder(results))
}

#[get("/{id}")]
pub async fn get_item(
    State(state): State<AppState>,
    rustapi::Path(id): rustapi::Path<i32>,
) -> Result<rustapi::Json<ItemResponse>, HTTPException> {
    info!("Accessed /items/{} route.", id);
    let items = state.items.lock().unwrap();
    let item = items.iter().find(|i| i.id == id).ok_or_else(|| {
        error!("Item {} not found.", id);
        HTTPException::new(StatusCode::NOT_FOUND, format!("Item {} not found", id))
    })?;

    Ok(rustapi::Json(ItemResponse {
        id: item.id,
        name: item.name.clone(),
        price: item.price,
    }))
}

#[post("")]
pub async fn create_item(
    State(state): State<AppState>,
    ValidatedJson(body): ValidatedJson<ItemCreate>,
) -> Response<rustapi::Json<ItemResponse>> {
    info!("Accessed /items route. Creating item: {}.", body.name);
    let mut items = state.items.lock().unwrap();
    let next_id = items.len() as i32 + 1;

    let db_item = ItemDb {
        id: next_id,
        name: body.name.clone(),
        price: body.price,
        internal_secret: std::env::var("INTERNAL_SECRET")
            .unwrap_or_else(|_| "development_secret".to_string()),
    };

    items.push(db_item.clone());

    Response::new(
        StatusCode::CREATED,
        rustapi::Json(ItemResponse {
            id: db_item.id,
            name: db_item.name,
            price: db_item.price,
        }),
    )
}

#[get("/error")]
pub async fn trigger_error() -> Result<&'static str, HTTPException> {
    error!("Accessed /error route. Triggering deliberate error.");
    Err(HTTPException::new(
        StatusCode::BAD_REQUEST,
        "This is a deliberate error",
    ))
}

#[get("/request-info")]
pub async fn request_info(req: rustapi::Request) -> String {
    info!(
        "Accessed /request-info route. Method: {}, Path: {}",
        req.method(),
        req.uri().path()
    );
    format!(
        "Request method: {}, Path: {}",
        req.method(),
        req.uri().path()
    )
}

#[get("/html")]
pub async fn html_example() -> HTMLResponse {
    HTMLResponse::new("<h1>Hello from RustAPI HTML Response!</h1>")
}

#[get("/redirect")]
pub async fn redirect_example() -> RedirectResponse {
    RedirectResponse::to("/hello")
}

#[get("/file")]
pub async fn file_example() -> FileResponse {
    FileResponse::new("Cargo.toml")
}

#[get("/background")]
pub async fn background_handler(tasks: BackgroundTasks) -> rustapi::Json<rustapi::Value> {
    info!("Accessed /background route. Scheduling tasks.");
    tasks.add_task(async {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("tasks.log")
            .unwrap();
        writeln!(file, "Task 1 starting...").unwrap();
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        writeln!(file, "Task 1 completed!").unwrap();
        file.flush().unwrap();
    });

    tasks.add_task(async {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("tasks.log")
            .unwrap();
        writeln!(file, "Task 2 starting...").unwrap();
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        writeln!(file, "Task 2 completed!").unwrap();
        file.flush().unwrap();
    });

    rustapi::Json(rustapi::json!({
        "message": "Background tasks scheduled"
    }))
}

#[post("/upload")]
pub async fn upload_handler(file: UploadFile) -> rustapi::Json<rustapi::Value> {
    let filename = file
        .filename
        .clone()
        .unwrap_or_else(|| "unknown.txt".to_string());
    info!("Accessed /upload route. Uploading file: {}.", filename);
    let size = file.content.len();

    // Save the file
    if let Err(e) = file.save(&filename).await {
        return rustapi::Json(rustapi::json!({
            "error": format!("Failed to save file: {}", e)
        }));
    }

    rustapi::Json(rustapi::json!({
        "message": "File uploaded successfully",
        "filename": filename,
        "size": size,
        "content_type": file.content_type
    }))
}

pub fn items_router() -> APIRouter<AppState> {
    APIRouter::new()
        .prefix("/items")
        .tag("items")
        .route(list_items_route())
        .route(get_item_route())
        .route(create_item_route())
        .route(request_info_route().with_state::<AppState>())
        .route(html_example_route().with_state::<AppState>())
        .route(redirect_example_route().with_state::<AppState>())
        .route(file_example_route().with_state::<AppState>())
}

pub fn auth_router() -> APIRouter<AppState> {
    APIRouter::new()
        .route(login_route().with_state::<AppState>())
        .route(me_route().with_state::<AppState>())
}
