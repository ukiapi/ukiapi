use axum::Router;
use serde_json::Value;
use crate::routing::api::RustAPI;

/// A type alias for a boxed closure that adds a route to an axum Router.
pub type RouteAdder<S> = Box<dyn FnOnce(Router<S>, &str) -> Router<S> + Send>;

/// Trait for items that can be added to the RustAPI router.
pub trait Routable<S> {
    /// Add the routes defined by this item to the RustAPI instance.
    fn add_to_api(self, api: &mut RustAPI<S>);
}

/// Represents a single API route.
pub struct Route<S = ()> {
    /// The HTTP method for this route (e.g., "GET", "POST").
    pub method: &'static str,
    /// The path for this route.
    pub path: String,
    /// The closure used to add this route to an axum Router.
    pub adder: RouteAdder<S>,
    /// The request schema for OpenAPI documentation.
    pub request_schema: Option<Value>,
    /// The response schema for OpenAPI documentation.
    pub response_schema: Option<Value>,
    /// The query parameters schema for OpenAPI documentation.
    pub query_schema: Option<Value>,
    /// Tags for OpenAPI documentation grouping.
    pub tags: Vec<String>,
}

impl<S> Routable<S> for Route<S> {
    fn add_to_api(self, api: &mut RustAPI<S>) {
        api.routes.push(self);
    }
}

impl<S> Route<S>
where
    S: Clone + Send + Sync + 'static,
{
    pub(crate) fn new<H, T>(method: &'static str, path: &'static str, handler: H) -> Self
    where
        H: axum::handler::Handler<T, S>,
        T: 'static,
    {
        let adder: RouteAdder<S> = match method {
            "GET" => Box::new(move |router, path| router.route(path, axum::routing::get(handler))),
            "POST" => Box::new(move |router, path| router.route(path, axum::routing::post(handler))),
            "PUT" => Box::new(move |router, path| router.route(path, axum::routing::put(handler))),
            "DELETE" => Box::new(move |router, path| router.route(path, axum::routing::delete(handler))),
            "PATCH" => Box::new(move |router, path| router.route(path, axum::routing::patch(handler))),
            _ => unreachable!(),
        };
        Self {
            method,
            path: path.to_string(),
            adder,
            request_schema: None,
            response_schema: None,
            query_schema: None,
            tags: Vec::new(),
        }
    }

    /// Set the request schema for OpenAPI documentation.
    pub fn with_request_schema(mut self, schema: Value) -> Self {
        self.request_schema = Some(schema);
        self
    }

    /// Set the response schema for OpenAPI documentation.
    pub fn with_response_schema(mut self, schema: Value) -> Self {
        self.response_schema = Some(schema);
        self
    }

    /// Set the query parameters schema for OpenAPI documentation.
    pub fn with_query_schema(mut self, schema: Value) -> Self {
        self.query_schema = Some(schema);
        self
    }

    /// Add a tag to this route for OpenAPI documentation grouping.
    pub fn tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string());
        self
    }

    /// Create a new GET route.
    pub fn get<H, T>(path: &'static str, handler: H) -> Self
    where
        H: axum::handler::Handler<T, S>,
        T: 'static,
    {
        Self::new("GET", path, handler)
    }

    /// Create a new POST route.
    pub fn post<H, T>(path: &'static str, handler: H) -> Self
    where
        H: axum::handler::Handler<T, S>,
        T: 'static,
    {
        Self::new("POST", path, handler)
    }

    /// Create a new PUT route.
    pub fn put<H, T>(path: &'static str, handler: H) -> Self
    where
        H: axum::handler::Handler<T, S>,
        T: 'static,
    {
        Self::new("PUT", path, handler)
    }

    /// Create a new DELETE route.
    pub fn delete<H, T>(path: &'static str, handler: H) -> Self
    where
        H: axum::handler::Handler<T, S>,
        T: 'static,
    {
        Self::new("DELETE", path, handler)
    }

    /// Create a new PATCH route.
    pub fn patch<H, T>(path: &'static str, handler: H) -> Self
    where
        H: axum::handler::Handler<T, S>,
        T: 'static,
    {
        Self::new("PATCH", path, handler)
    }
}

impl Route<()> {
    /// Convert this stateless route to a stateful one.
    pub fn with_state<NewS>(self) -> Route<NewS>
    where
        NewS: Clone + Send + Sync + 'static,
    {
        let old_adder = self.adder;
        let new_adder: RouteAdder<NewS> = Box::new(move |router: Router<NewS>, path| {
            let stateless_router = Router::<()>::new();
            let built_stateless_router = old_adder(stateless_router, path);
            router.merge(built_stateless_router.with_state(()))
        });
        Route {
            method: self.method,
            path: self.path,
            adder: new_adder,
            request_schema: self.request_schema,
            response_schema: self.response_schema,
            query_schema: self.query_schema,
            tags: self.tags,
        }
    }
}
