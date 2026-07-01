# ukidama

Core library for the Ukidama framework. Provides routing, extractors, and utilities for building FastAPI-inspired APIs in Rust.

## 📦 Key Components

### 1. Extractors
- `ValidatedJson<T>`: Extracts and validates JSON bodies using `validator`.
- `Query<T>`: Extracts and validates query parameters.

### 2. Error Handling
- `HTTPException`: A structured way to return HTTP errors with status codes and messages.
  ```rust
  use ukidama::HTTPException;
  use ukidama::http::StatusCode;

  pub async fn handler() -> Result<&'static str, HTTPException> {
      Err(HTTPException::new(StatusCode::BAD_REQUEST, "Invalid input"))
  }
  ```

### 3. Response Wrapping
- `Response<T>`: A wrapper to explicitly set status codes for responses.
  ```rust
  use ukidama::Response;
  use ukidama::http::StatusCode;

  pub async fn handler() -> Response<String> {
      Response::new(StatusCode::CREATED, "Item created".to_string())
  }
  ```

### 4. Routing
- `APIRouter`: Modularize your API by grouping routes with prefixes and tags.
- `Ukidama`: The main entry point to build your `axum::Router`.
- `routes!` macro: A convenient way to initialize your API with multiple routes.

## 🛠️ Usage Example

```rust
use ukidama::{get, routes, APIRouter, AppState};

#[get("/world")]
async fn hello_world() -> &'static str {
    "Hello World"
}

fn my_router() -> APIRouter<AppState> {
    APIRouter::new()
        .prefix("/hello")
        .tag("greeting")
        .route(hello_world_route())
}
```
