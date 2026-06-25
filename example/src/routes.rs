use rustapi::{get, post, json, Value, ValidatedJson};
use axum::extract::State;
use crate::models::Item;
use crate::AppState;

#[get("/hello")]
pub async fn hello() -> &'static str {
    "Hello from RustAPI!"
}

#[get("/items")]
pub async fn list_items(State(state): State<AppState>) -> rustapi::Json<Vec<Item>> {
    let items = state.items.lock().unwrap();
    rustapi::Json(items.clone())
}

#[get("/items/{id}")]
pub async fn get_item(rustapi::Path(id): rustapi::Path<i32>) -> rustapi::Json<Value> {
    rustapi::Json(json!({"id": id, "name": "item", "price": 9.99}))
}

#[post("/items")]
pub async fn create_item(
    State(state): State<AppState>,
    ValidatedJson(body): ValidatedJson<Item>,
) -> rustapi::Json<Item> {
    state.items.lock().unwrap().push(body.clone());
    rustapi::Json(body)
}
