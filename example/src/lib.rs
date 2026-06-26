pub mod models;
pub mod routes;

use crate::models::ItemDb;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct AppState {
    pub items: Arc<Mutex<Vec<ItemDb>>>,
}
