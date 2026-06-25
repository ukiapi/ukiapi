use std::sync::{Arc, Mutex};
use rustapi::{get, post, model, json, Value, ValidatedJson};
use axum::extract::State;
use serde::{Deserialize, Serialize};

#[model]
pub struct Item {
    pub name: String,
    pub price: f64,
}

#[derive(Clone)]
pub struct AppState {
    pub items: Arc<Mutex<Vec<Item>>>,
}

#[get("/items")]
pub async fn list_items(State(state): State<AppState>) -> rustapi::Json<Vec<Item>> {
    unimplemented!()
}

#[post("/items")]
pub async fn create_item(
    State(state): State<AppState>,
    ValidatedJson(body): ValidatedJson<Item>,
) -> rustapi::Json<Item> {
    unimplemented!()
}

#[tokio::main]
async fn main() {
    let state = AppState { items: Arc::new(Mutex::new(Vec::new())) };

    let app = rustapi::routes![AppState,
        list_items_route(),
        create_item_route()
    ].build_router(state);
    
    // We can't easily extract openapi_json from the router. 
    // Let's just print schemars::schema_for!(Item) and schemars::schema_for!(Vec<Item>)
    println!("Item: {}", serde_json::to_string_pretty(&rustapi::schema_for::<Item>()).unwrap());
    println!("Vec<Item>: {}", serde_json::to_string_pretty(&rustapi::schema_for::<Vec<Item>>()).unwrap());
}
