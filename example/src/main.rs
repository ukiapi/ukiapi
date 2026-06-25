mod models;
mod routes;

use std::sync::{Arc, Mutex};
use crate::models::ItemDb;
use crate::routes::*;

#[derive(Clone)]
pub struct AppState {
    pub items: Arc<Mutex<Vec<ItemDb>>>,
}

#[tokio::main]
async fn main() {
    let state = AppState { items: Arc::new(Mutex::new(Vec::new())) };

    let app = rustapi::routes![AppState,
        hello_route().with_state::<AppState>(),
        list_items_route(),
        get_item_route().with_state::<AppState>(),
        create_item_route()
    ].build_router(state);

    rustapi::serve(app).await;
}
