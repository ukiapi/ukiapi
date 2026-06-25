mod models;
mod routes;

use crate::models::ItemDb;
use crate::routes::*;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct AppState {
    pub items: Arc<Mutex<Vec<ItemDb>>>,
}

#[tokio::main]
async fn main() {
    let state = AppState {
        items: Arc::new(Mutex::new(Vec::new())),
    };

    let app = rustapi::routes![
        AppState,
        hello_route().with_state::<AppState>(),
        list_items_route(),
        get_item_route(),
        create_item_route(),
        trigger_error_route().with_state::<AppState>()
    ]
    .build_router(state);

    rustapi::serve(app).await;
}
