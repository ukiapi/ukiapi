# Learn

Tutorials and guides to help you get started with UkiApi.

## Prerequisites

- Rust 1.70+ (recommended: latest stable)
- Cargo

## Installation

```bash
cargo install ukiapi
```

## Tutorial 1: Hello World

### Step 1: Create a new project

```bash
ukiapi new hello-api
cd hello-api
```

### Step 2: Edit `src/main.rs`

```rust
use ukiapi::{get, routes};

#[get("/hello")]
async fn hello() -> &'static str {
    "Hello, World!"
}

#[ukiapi::main]
async fn main() {
    routes![(), hello()]
        .serve(())
        .await;
}
```

### Step 3: Run the server

```bash
ukiapi dev
```

Visit `http://localhost:3000/hello` to see your response.

## Tutorial 2: REST API with Validation

### Step 1: Define your models

```rust
use serde::{Deserialize, Serialize};
use validator::Validate;
use schemars::JsonSchema;
use ukiapi::Projected;

#[derive(Clone, Serialize, Deserialize, Validate, JsonSchema)]
pub struct Todo {
    #[serde(skip_deserializing)]
    pub id: Option<i64>,
    
    #[validate(length(min = 1, max = 200))]
    pub title: String,
    
    pub completed: bool,
}

#[derive(Deserialize, Validate, JsonSchema)]
pub struct CreateTodo {
    #[validate(length(min = 1, max = 200))]
    pub title: String,
}
```

### Step 2: Create routes

```rust
use ukiapi::{get, post, patch, delete, ValidatedJson, Path, Json, routes};
use serde_json::{json, Value};

#[get("/todos")]
async fn list_todos() -> Json<Vec<Todo>> {
    // Database query here
    Json(vec![])
}

#[post("/todos")]
async fn create_todo(ValidatedJson(todo): ValidatedJson<CreateTodo>) -> Json<Todo> {
    Json(Todo {
        id: Some(1),
        title: todo.title,
        completed: false,
    })
}

#[patch("/todos/{id}")]
async fn update_todo(
    Path(id): Path<i64>,
    ValidatedJson(todo): ValidatedJson<CreateTodo>,
) -> Json<Todo> {
    Json(Todo {
        id: Some(id),
        title: todo.title,
        completed: false,
    })
}

#[delete("/todos/{id}")]
async fn delete_todo(Path(id): Path<i64>) -> Json<Value> {
    Json(json!({ "deleted": id }))
}
```

### Step 3: Wire it up

```rust
#[ukiapi::main]
async fn main() {
    routes![(),
        list_todos(),
        create_todo(),
        update_todo(),
        delete_todo()
    ]
    .title("Todo API")
    .version("1.0.0")
    .serve(())
    .await;
}
```

## Tutorial 3: Database Integration

### Using SQLx with SQLite

```rust
use sqlx::SqlitePool;

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
}

async fn init_db() -> SqlitePool {
    SqlitePool::connect("sqlite:todos.db?mode=rwc")
        .await
        .expect("Failed to connect to database")
}

#[ukiapi::main]
async fn main() {
    let db = init_db().await;
    
    // Run migrations
    sqlx::query("CREATE TABLE IF NOT EXISTS todos (id INTEGER PRIMARY KEY, title TEXT, completed BOOLEAN)")
        .execute(&db)
        .await
        .expect("Failed to create table");
    
    let state = AppState { db };
    
    routes![AppState, /* routes */]
        .serve(state)
        .await;
}
```

## Tutorial 4: Authentication

### JWT Authentication

```rust
use ukiapi::{JWTAuth, encode_jwt, Depends};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i64,
    pub exp: usize,
}

#[post("/login")]
async fn login(Json(credentials): Json<LoginRequest>) -> Json<Value> {
    // Validate credentials...
    let claims = Claims { sub: user_id, exp: chrono::Utc::now() + chrono::Duration::hours(24) };
    let token = encode_jwt(&claims, "secret").unwrap();
    Json(json!({ "access_token": token }))
}

#[get("/protected")]
async fn protected(auth: JWTAuth) -> Json<Value> {
    Json(json!({ "user_id": auth.claims.sub }))
}
```

## Next Steps

- Read the [API Reference](reference.md) for detailed documentation
- Check out the [Full Stack Template](resources.md#full-stack-template)
- Explore the [example](https://github.com/ukiapi/ukiapi/tree/main/example) project
