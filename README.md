# 🦀 RustAPI

**RustAPI** is a high-performance web framework for Rust, built on top of [Axum](https://github.com/tokio-rs/axum). It is designed to provide a developer experience similar to [FastAPI](https://fastapi.tiangolo.com/), focusing on ergonomics, automatic documentation, and type-safe data validation.

## ✨ Features

- 🚀 **High Performance:** Built on Axum and Tokio.
- 📝 **Automatic Documentation:** Built-in Swagger UI and ReDoc (via `schemars` and custom macros).
- ✅ **Data Validation:** Seamless integration with `validator` for request and query parameters.
- 🔗 **Type-Safe Routing:** Declarative routing using macros like `#[get]`, `#[post]`, etc.
- 🔁 **Hot Reload:** Automatic recompilation on file changes via `--reload`.
- 🧩 **Middleware:** Custom middleware, CORS, logging, compression, and timeout support.
- 📁 **Static Files & Uploads:** Built-in file serving and multipart upload handling.
- ⏳ **Background Tasks:** Schedule async tasks to run after the response is sent.
- 📦 **FastAPI-like DX:** Familiar patterns for developers coming from Python.
- 🛠️ **CLI Tooling:** Dedicated CLI for scaffolding and development.

## 🚀 Quick Start

### 1. Install the CLI

```bash
cargo install --git ssh://git@github.com/abundis29/rustapi.git rustapi-cli --tag v0.1.1
```

### 2. Create a new project

```bash
rustapi new my-api
cd my-api
```

### 3. Run in development mode

```bash
rustapi dev
```

Or with hot reload:

```bash
rustapi dev --reload
```

Your API will be running at `http://localhost:3000`, with interactive docs at `http://localhost:3000/docs`.

## 📖 Example

```rust
use rustapi::{get, post, routes, ValidatedJson, Json, json};
use serde::{Deserialize, Serialize};
use validator::Validate;
use schemars::JsonSchema;

#[derive(Clone)]
pub struct AppState {}

#[derive(Serialize, Deserialize, Validate, JsonSchema)]
pub struct CreateItem {
    #[validate(length(min = 1))]
    name: String,
}

#[get("/hello")]
pub async fn hello() -> &'static str {
    "Hello from RustAPI!"
}

#[post("/items")]
pub async fn create_item(ValidatedJson(payload): ValidatedJson<CreateItem>) -> Json<serde_json::Value> {
    Json(json!({ "item": payload.name }))
}

#[tokio::main]
async fn main() {
    let state = AppState {};

    routes![AppState,
        hello_route().with_state::<AppState>(),
        create_item_route().with_state::<AppState>()
    ]
    .serve(state)
    .await;
}
```

## 🏗️ Project Structure

- `rustapi`: Core library providing routing, extractors, and types.
- `rustapi-macros`: Procedural macros for routing and models.
- `rustapi-cli`: Command-line interface for managing RustAPI projects.
- `example`: A reference implementation demonstrating all features.

## ⚖️ License

MIT OR Apache-2.0
