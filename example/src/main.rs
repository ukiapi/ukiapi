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

    rustapi::routes![
        AppState,
        hello_route().with_state::<AppState>(),
        items_router(),
        trigger_error_route().with_state::<AppState>()
    ]
    .title("Example API")
    .version("1.0.0")
    .on_startup(|_state| async {
        println!("🚀 Application starting up...");
    })
    .on_shutdown(|_state| async {
        println!("🛑 Application shutting down...");
    })
    .mount("/static", ".")
    .serve(state)
    .await;
}
