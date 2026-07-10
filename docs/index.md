# UkiApi Documentation

[![Tests](https://github.com/ukiapi/ukiapi/actions/workflows/ci.yml/badge.svg)](https://github.com/ukiapi/ukiapi/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**UkiApi** is a high-performance web framework for Rust, built on top of [Axum](https://github.com/tokio-rs/axum), focusing on ergonomics, automatic documentation, and type-safe data validation.

---

## Quick Start

```bash
# Install the CLI
cargo install ukiapi

# Create a new project
ukiapi new my-api
cd my-api

# Run in development mode
ukiapi dev
```

Your API will be running at `http://localhost:3000`, with interactive docs at `http://localhost:3000/docs`.

---

## Documentation

| Section | Description |
|---------|-------------|
| [Introduction](introduction.md) | Overview and philosophy |
| [Features](features.md) | What UkiApi offers |
| [Learn](learn.md) | Tutorials and guides |
| [Reference](reference.md) | API reference |
| [Resources](resources.md) | Templates and examples |
| [Release Notes](release-notes.md) | Version history |
| [About](about.md) | Project information |

---

## Example

```rust
use ukiapi::{get, post, routes, ValidatedJson};
use ukiapi::Json;
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
    "Hello from UkiApi!"
}

#[post("/items")]
pub async fn create_item(ValidatedJson(payload): ValidatedJson<CreateItem>) -> Json<serde_json::Value> {
    Json(json!({ "item": payload.name }))
}

#[ukiapi::main]
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
