pub mod api;
pub mod middleware;
pub mod route;
pub mod router;

pub use api::RustAPI;
pub use axum::Router;
pub use route::{Routable, Route, RouteAdder};
pub use router::APIRouter;
use std::sync::Arc;

pub mod methods {
    pub use axum::routing::{delete, get, patch, post, put};
}

pub type RouterBuilder<S> = Arc<dyn Fn(Router<S>) -> Router<S> + Send + Sync>;
