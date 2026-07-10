# Reference

Detailed API reference for UkiApi.

## Macros

### Route Macros

| Macro | Description |
|-------|-------------|
| `#[get("/path")]` | Define a GET route |
| `#[post("/path")]` | Define a POST route |
| `#[put("/path")]` | Define a PUT route |
| `#[patch("/path")]` | Define a PATCH route |
| `#[delete("/path")]` | Define a DELETE route |
| `#[websocket("/path")]` | Define a WebSocket route |

### Main Macro

```rust
#[ukiapi::main]
async fn main() {
    // Automatically sets UKIAPI_HOST and UKIAPI_PORT
    // Initializes env_logger
}
```

### Model Macro

```rust
#[derive(ukiapi::model)]
pub struct User {
    pub id: i64,
    pub name: String,
}
```

## Extractors

| Extractor | Description |
|-----------|-------------|
| `Path<T>` | Extract path parameters |
| `Query<T>` | Extract query parameters |
| `Json<T>` | Extract JSON body |
| `ValidatedJson<T>` | Extract and validate JSON body |
| `State<T>` | Extract application state |
| `Request` | Extract the full request |
| `UploadFile` | Extract multipart file upload |
| `WebSocketUpgrade` | Upgrade to WebSocket connection |

## Response Types

| Type | Description |
|------|-------------|
| `Json<T>` | JSON response |
| `Html<T>` | HTML response |
| `StatusCode` | Status code only |
| `Response` | Custom response |
| `Projected<T>` | Field projection |

## Middleware

### Built-in Layers

```rust
use ukiapi::routing::middleware::layers::{
    CorsLayer,
    CompressionLayer,
    TraceLayer,
    TimeoutLayer,
    DefaultBodyLimit,
};
```

### MiddlewareExt Trait

```rust
use ukiapi::routing::middleware::MiddlewareExt;

router
    .cors(cors_layer)
    .compression()
    .logger()
    .timeout(Duration::from_secs(30))
    .body_limit(10 * 1024 * 1024)
    .middleware(custom_fn)
```

## Router Builder

```rust
routes![AppState, route1(), route2()]
    .title("My API")
    .version("1.0.0")
    .cors(cors)
    .compression()
    .logger()
    .serve(state)
    .await;
```

## Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `UKIAPI_HOST` | `127.0.0.1` | Server host |
| `UKIAPI_PORT` | `3000` | Server port |
| `RUST_LOG` | - | Log level filter |

## CLI Commands

| Command | Description |
|---------|-------------|
| `uki new <name>` | Create a new project |
| `uki dev` | Run in development mode |
| `uki dev --reload` | Run with hot reload |
| `uki run` | Run the server |

## Error Handling

UkiApi uses `HTTPException` for error responses:

```rust
use ukiapi::response::HTTPException;

#[get("/users/{id}")]
async fn get_user(Path(id): Path<i64>) -> Result<Json<User>, HTTPException> {
    let user = db::find_user(id).await
        .ok_or(HTTPException::not_found("User not found"))?;
    Ok(Json(user))
}
```

### Status Codes

```rust
HTTPException::bad_request("Invalid input")
HTTPException::unauthorized("Not authenticated")
HTTPException::forbidden("Access denied")
HTTPException::not_found("Resource not found")
HTTPException::conflict("Already exists")
HTTPException::internal_error("Server error")
```
