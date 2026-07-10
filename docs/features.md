# Features

UkiApi provides a comprehensive set of features for building production-ready APIs.

## Core Features

### Routing

```rust
use ukiapi::{get, post, put, patch, delete};

#[get("/users")]
async fn list_users() -> Json<Vec<User>> { /* ... */ }

#[post("/users")]
async fn create_user(ValidatedJson(user): ValidatedJson<NewUser>) -> Json<User> { /* ... */ }

#[get("/users/{id}")]
async fn get_user(Path(id): Path<i64>) -> Json<User> { /* ... */ }

#[put("/users/{id}")]
async fn update_user(Path(id): Path<i64>, ValidatedJson(user): ValidatedJson<UpdateUser>) -> Json<User> { /* ... */ }

#[delete("/users/{id}")]
async fn delete_user(Path(id): Path<i64>) -> StatusCode { /* ... */ }
```

### Validation

```rust
use validator::Validate;
use schemars::JsonSchema;

#[derive(Serialize, Deserialize, Validate, JsonSchema)]
pub struct CreateUser {
    #[validate(length(min = 1, max = 100))]
    name: String,
    
    #[validate(email)]
    email: String,
    
    #[validate(range(min = 18))]
    age: u32,
}
```

### Authentication

```rust
use ukiapi::{JWTAuth, Depends};

#[get("/protected")]
async fn protected_route(auth: JWTAuth) -> Json<Value> {
    Json(json!({ "user_id": auth.claims.sub }))
}
```

### WebSocket

```rust
use ukiapi::{websocket, WebSocket, WebSocketUpgrade};

#[websocket("/ws")]
async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket))
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(Ok(msg)) = socket.recv().await {
        socket.send(msg).await.ok();
    }
}
```

## Middleware

### Built-in Middleware

```rust
use ukiapi::routing::middleware::layers::CorsLayer;

routes![AppState, /* routes */]
    .cors(CorsLayer::permissive())
    .compression()
    .logger()
    .timeout(Duration::from_secs(30))
    .body_limit(10 * 1024 * 1024) // 10MB
    .serve(state)
    .await;
```

### Custom Middleware

```rust
async fn my_middleware(req: Request, next: Next) -> Response {
    println!("Request: {}", req.uri());
    let response = next.run(req).await;
    println!("Response: {}", response.status());
    response
}

routes![AppState, /* routes */]
    .middleware(my_middleware)
    .serve(state)
    .await;
```

## Background Tasks

```rust
use ukiapi::BackgroundTasks;

#[post("/process")]
async fn process(
    State(state): State<AppState>,
    tasks: BackgroundTasks,
) -> Json<Value> {
    tasks.spawn(async move {
        // Long-running task
        expensive_operation().await;
    });
    
    Json(json!({ "status": "processing" }))
}
```

## Static Files

```rust
use ukiapi::routing::api::ServeDir;

// Serve files from ./static directory
routes![AppState, /* routes */]
    .static_files("/static", ServeDir::new("static"))
    .serve(state)
    .await;
```

## File Uploads

```rust
use ukiapi::UploadFile;

#[post("/upload")]
async fn upload(UploadFile(file): UploadFile) -> Json<Value> {
    let contents = file.bytes().await?;
    Json(json!({ "size": contents.len() }))
}
```

## Response Projection

```rust
use ukiapi::Projected;

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub password_hash: String, // Never exposed
}

#[get("/users/{id}")]
async fn get_user(Path(id): Path<i64>) -> Json<Projected<User>> {
    let user = db::find_user(id).await?;
    // Only expose id and name fields
    Ok(Json(user.project(|u| (u.id, u.name))))
}
```
