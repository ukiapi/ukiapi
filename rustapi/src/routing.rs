pub mod api;
pub mod route;
pub mod router;

pub use api::RustAPI;
pub use route::{Routable, Route, RouteAdder};
pub use router::APIRouter;
use std::sync::Arc;
use axum::Router;

pub type RouterBuilder<S> = Arc<dyn Fn(Router<S>) -> Router<S> + Send + Sync>;
