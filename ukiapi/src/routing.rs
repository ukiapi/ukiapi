pub mod api;
pub mod autodiscover;
pub mod middleware;
pub mod route;
pub mod router;

pub use api::UkiApi;
pub use autodiscover::{DefaultRoute, RegistryEntry};
pub use axum::Router;
pub use route::{Routable, Route, RouteAdder};
pub use router::APIRouter;
use std::sync::Arc;

pub mod methods {
    pub use axum::routing::{any, delete, get, patch, post, put};
}

pub type RouterBuilder<S> = Arc<dyn Fn(Router<S>) -> Router<S> + Send + Sync>;
