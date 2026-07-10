use crate::models::{
    ItemCreate, ItemDb, ItemResponse, ListItemsQuery, LoginRequest, TokenResponse, UserClaims,
};
use crate::AppState;
use std::fs::OpenOptions;
use std::io::Write;
use ukidama::http::StatusCode;
use ukidama::State;
use ukidama::{
    encode_jwt, error, get, info, jsonable_encoder, post, APIRouter, BackgroundTasks, Depends,
    FileResponse, HTMLResponse, HTTPException, JWTAuth, Projected, RedirectResponse, Response,
    UploadFile, ValidatedJson,
};
use ukidama::{websocket, Message, WebSocket, WebSocketUpgrade};

ukidama::declare_registry!(crate::AppState, ItemsRoute);
ukidama::declare_registry!(crate::AppState, AuthRoute);

#[get("/hello")]
pub async fn hello() -> &'static str {
    info!("Accessed /hello route.");
    "Hello from Ukidama!"
}

#[post("/login", registry = AuthRoute)]
pub async fn login(
    ValidatedJson(body): ValidatedJson<LoginRequest>,
) -> Result<ukidama::Json<TokenResponse>, HTTPException> {
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

    let secret = std::env::var("JWT_SECRET").map_err(|_| {
        HTTPException::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "JWT_SECRET environment variable is not set",
        )
    })?;
    let token = encode_jwt(&claims, &secret).map_err(|e| {
        HTTPException::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to generate token: {}", e),
        )
    })?;

    Ok(ukidama::Json(TokenResponse {
        access_token: token,
        token_type: "Bearer".to_string(),
    }))
}

#[get("/me", registry = AuthRoute)]
pub async fn me(Depends(claims, _): Depends<JWTAuth<UserClaims>>) -> ukidama::Json<UserClaims> {
    ukidama::Json(claims)
}

#[get("", registry = ItemsRoute)]
pub async fn list_items(
    State(state): State<AppState>,
    ukidama::Query(query): ukidama::Query<ListItemsQuery>,
) -> ukidama::Json<ukidama::Value> {
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
    ukidama::Json(jsonable_encoder(results))
}

#[get("/{id}", registry = ItemsRoute)]
pub async fn get_item(
    State(state): State<AppState>,
    ukidama::Path(id): ukidama::Path<i32>,
) -> Result<Projected<ItemResponse>, HTTPException> {
    info!("Accessed /items/{} route.", id);
    let items = state.items.lock().unwrap();
    let item = items.iter().find(|i| i.id == id).ok_or_else(|| {
        error!("Item {} not found.", id);
        HTTPException::new(StatusCode::NOT_FOUND, format!("Item {} not found", id))
    })?;

    let response = ItemResponse {
        id: item.id,
        name: item.name.clone(),
        price: item.price,
    };

    // Demonstrate dynamic projection by excluding price
    Ok(Projected::new(response).exclude(vec!["price"]))
}

#[post("", registry = ItemsRoute)]
pub async fn create_item(
    State(state): State<AppState>,
    ValidatedJson(body): ValidatedJson<ItemCreate>,
) -> Response<ukidama::Json<ItemResponse>> {
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
        ukidama::Json(ItemResponse {
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

#[get("/request-info", registry = ItemsRoute)]
pub async fn request_info(req: ukidama::Request) -> String {
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

#[get("/html", registry = ItemsRoute)]
pub async fn html_example() -> HTMLResponse {
    HTMLResponse::new("<h1>Hello from Ukidama HTML Response!</h1>")
}

#[get("/redirect", registry = ItemsRoute)]
pub async fn redirect_example() -> RedirectResponse {
    RedirectResponse::to("/hello")
}

#[get("/file", registry = ItemsRoute)]
pub async fn file_example() -> FileResponse {
    FileResponse::new("Cargo.toml")
}

#[get("/background")]
pub async fn background_handler(tasks: BackgroundTasks) -> ukidama::Json<ukidama::Value> {
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

    ukidama::Json(ukidama::json!({
        "message": "Background tasks scheduled"
    }))
}

#[post("/upload")]
pub async fn upload_handler(file: UploadFile) -> ukidama::Json<ukidama::Value> {
    let filename = file
        .filename
        .clone()
        .unwrap_or_else(|| "unknown.txt".to_string());
    info!("Accessed /upload route. Uploading file: {}.", filename);
    let size = file.content.len();

    // 🛡️ Sentinel: Save to temp directory to prevent CWD file overwrite
    let dest = std::env::temp_dir().join(&filename);
    if let Err(e) = file.save(&dest).await {
        return ukidama::Json(ukidama::json!({
            "error": format!("Failed to save file: {}", e)
        }));
    }

    ukidama::Json(ukidama::json!({
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
        .autodiscover_with::<ItemsRoute>()
}

pub fn auth_router() -> APIRouter<AppState> {
    APIRouter::new().autodiscover_with::<AuthRoute>()
}

#[websocket("/ws")]
pub async fn ws_echo(ws: WebSocketUpgrade) -> ukidama::response::AxumResponse {
    info!("New WebSocket connection request.");
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    info!("WebSocket connection established.");
    while let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
            match msg {
                Message::Text(text) => {
                    info!("Received WebSocket message: {}", text);
                    if socket.send(Message::Text(text)).await.is_err() {
                        break;
                    }
                }
                Message::Binary(bin) => {
                    if socket.send(Message::Binary(bin)).await.is_err() {
                        break;
                    }
                }
                Message::Close(_) => break,
                _ => {}
            }
        } else {
            break;
        }
    }
    info!("WebSocket connection closed.");
}
