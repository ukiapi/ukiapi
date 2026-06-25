# 🦀 RustAPI

**RustAPI** is a high-performance web framework for Rust, built on top of [Axum](https://github.com/tokio-rs/axum). It is designed to provide a developer experience similar to [FastAPI](https://fastapi.tiangolo.com/), focusing on ergonomics, automatic documentation, and type-safe data validation.

## ✨ Features

- 🚀 **High Performance:** Built on Axum and Tokio.
- 📝 **Automatic Documentation:** Built-in Swagger UI and ReDoc (via `schemars` and custom macros).
- ✅ **Data Validation:** Seamless integration with `validator` for request and query parameters.
- 🔗 **Type-Safe Routing:** Declarative routing using macros like `#[get]`, `#[post]`, etc.
- 📦 **FastAPI-like DX:** Familiar patterns for developers coming from Python.
- 🛠️ **CLI Tooling:** Dedicated CLI for scaffolding and development.

## 🚀 Quick Start

### 1. Install the CLI

```bash
cargo install --path rustapi-cli
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

Your API will be running at `http://localhost:3000`, with interactive docs at `http://localhost:3000/docs`.

## 📖 Example

```rust
use rustapi::{get, post, routes, serve, ValidatedJson};
use axum::Json;
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
pub async fn create_item(ValidatedJson(payload): ValidatedJson<CreateItem>) -> Json<CreateItem> {
    Json(payload)
}

#[tokio::main]
async fn main() {
    let state = AppState {};
    let app = routes![AppState, hello(), create_item()].build_router(state);
    serve(app).await;
}
```

## 🏗️ Project Structure

- `rustapi`: Core library providing routing, extractors, and types.
- `rustapi-macros`: Procedural macros for routing and models.
- `rustapi-cli`: Command-line interface for managing RustAPI projects.
- `example`: A reference implementation demonstrating features.

## ⚖️ License

MIT OR Apache-2.0
