use rustapi::{get, post, json, Value, ValidatedJson};
use axum::extract::State;
use crate::models::{ItemDb, ItemCreate, ItemResponse, ListItemsQuery};
use crate::AppState;

#[get("/hello")]
pub async fn hello() -> &'static str {
    "Hello from RustAPI!"
}

#[get("/items")]
pub async fn list_items(
    State(state): State<AppState>,
    rustapi::Query(query): rustapi::Query<ListItemsQuery>,
) -> rustapi::Json<Vec<ItemResponse>> {
    let items = state.items.lock().unwrap();
    let limit = query.limit.unwrap_or(10) as usize;
    
    let mut results: Vec<ItemResponse> = items.iter()
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
    rustapi::Json(results)
}

#[get("/items/{id}")]
pub async fn get_item(rustapi::Path(id): rustapi::Path<i32>) -> rustapi::Json<Value> {
    rustapi::Json(json!({"id": id, "name": "item", "price": 9.99}))
}

#[post("/items")]
pub async fn create_item(
    State(state): State<AppState>,
    ValidatedJson(body): ValidatedJson<ItemCreate>,
) -> rustapi::Json<ItemResponse> {
    let mut items = state.items.lock().unwrap();
    let next_id = items.len() as i32 + 1;
    
    let db_item = ItemDb {
        id: next_id,
        name: body.name.clone(),
        price: body.price,
        internal_secret: "secret123".to_string(),
    };
    
    items.push(db_item.clone());
    
    rustapi::Json(ItemResponse {
        id: db_item.id,
        name: db_item.name,
        price: db_item.price,
    })
}
