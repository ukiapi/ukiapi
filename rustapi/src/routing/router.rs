use crate::routing::api::RustAPI;
use crate::routing::route::{Routable, Route};

/// A type alias for a boxed closure that builds an axum Router.
pub type RouterBuilder<S> = Box<dyn FnOnce(axum::Router<S>) -> axum::Router<S> + Send>;

/// A router for grouping routes with a common prefix and tags.
pub struct APIRouter<S = ()> {
    /// The prefix for all routes in this router.
    pub prefix: String,
    /// Tags for all routes in this router.
    pub tags: Vec<String>,
    /// The list of routes in this router.
    pub routes: Vec<Route<S>>,
}

impl<S> Default for APIRouter<S>
where
    S: Clone + Send + Sync + 'static,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<S> APIRouter<S>
where
    S: Clone + Send + Sync + 'static,
{
    /// Create a new APIRouter.
    pub fn new() -> Self {
        Self {
            prefix: String::new(),
            tags: Vec::new(),
            routes: Vec::new(),
        }
    }

    /// Set a common prefix for all routes in this router.
    pub fn prefix(mut self, prefix: &str) -> Self {
        self.prefix = prefix.to_string();
        self
    }

    /// Add a common tag for all routes in this router.
    pub fn tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string());
        self
    }

    /// Add a route or another router to this router.
    pub fn route<R: Routable<S>>(mut self, routable: R) -> Self {
        let mut temp_api = RustAPI::<S>::new();
        routable.add_to_api(&mut temp_api);
        for mut route in temp_api.routes {
            route.path = format!("{}{}", self.prefix, route.path);
            route.tags.extend(self.tags.clone());
            self.routes.push(route);
        }
        self
    }
}

impl<S> Routable<S> for APIRouter<S> {
    fn add_to_api(self, api: &mut RustAPI<S>) {
        api.routes.extend(self.routes);
    }
}
