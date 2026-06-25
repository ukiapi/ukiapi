use rustapi::{get, post, model, json, Value, ValidatedJson};
use axum::extract::State;
use std::sync::{Arc, Mutex};

#[model]
struct Item {
    name: String,
    price: f64,
}

#[derive(Clone)]
struct AppState {
    items: Arc<Mutex<Vec<Item>>>,
}


#[get("/hello")]
async fn hello() -> &'static str {
    "Hello from RustAPI!"
}

#[get("/items")]
async fn list_items(State(state): State<AppState>) -> rustapi::Json<Vec<Item>> {
    let items = state.items.lock().unwrap();
    rustapi::Json(items.clone())
}

#[get("/items/{id}")]
async fn get_item(rustapi::Path(id): rustapi::Path<i32>) -> rustapi::Json<Value> {
    rustapi::Json(json!({"id": id, "name": "item", "price": 9.99}))
}

#[post("/items")]
async fn create_item(
    State(state): State<AppState>,
    ValidatedJson(body): ValidatedJson<Item>,
) -> rustapi::Json<Item> {
    state.items.lock().unwrap().push(body.clone());
    rustapi::Json(body)
}

#[tokio::main]
async fn main() {
    let state = AppState { items: Arc::new(Mutex::new(Vec::new())) };

    let app = rustapi::routes![AppState,
        hello_route().with_state::<AppState>(),
        list_items_route(),
        get_item_route().with_state::<AppState>(),
        create_item_route()
    ]
        .build_router(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}
