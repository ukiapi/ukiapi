pub mod api;
pub mod middleware;
pub mod route;
pub mod router;

pub use api::RustAPI;
use axum::Router;
pub use middleware::MiddlewareExt;
pub use route::{Routable, Route, RouteAdder};
pub use router::APIRouter;
use std::sync::Arc;

pub type RouterBuilder<S> = Arc<dyn Fn(Router<S>) -> Router<S> + Send + Sync>;
