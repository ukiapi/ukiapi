use serde_json::Value;
use crate::routing::{RouteAdder, Ukidama, Routable};
use crate::handler::Handler;
use crate::routing::{methods, Router};

pub struct Route<S = ()> {
    pub method: &'static str,
    pub path: String,
    pub adder: RouteAdder<S>,
    pub request_schema: Option<Value>,
    pub response_schema: Option<Value>,
    pub query_schema: Option<Value>,
    pub tags: Vec<String>,
}

impl<S> Routable<S> for Route<S> {
    fn add_to_api(self, api: &mut Ukidama<S>) {
        api.routes.push(self);
    }
}

impl<S> Route<S>
where
    S: Clone + Send + Sync + 'static,
{
    pub fn new<H, T>(method: &'static str, path: &'static str, handler: H) -> Self
    where
        H: Handler<T, S>,
        T: 'static,
    {
        let adder: RouteAdder<S> = match method {
            "GET" => Box::new(move |router, path| router.route(path, methods::get(handler))),
            "POST" => {
                Box::new(move |router, path| router.route(path, methods::post(handler)))
            }
            "PUT" => Box::new(move |router, path| router.route(path, methods::put(handler))),
            "DELETE" => {
                Box::new(move |router, path| router.route(path, methods::delete(handler)))
            }
            "PATCH" => {
                Box::new(move |router, path| router.route(path, methods::patch(handler)))
            }
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

    pub fn with_request_schema(mut self, schema: Value) -> Self {
        self.request_schema = Some(schema);
        self
    }

    pub fn with_response_schema(mut self, schema: Value) -> Self {
        self.response_schema = Some(schema);
        self
    }

    pub fn with_query_schema(mut self, schema: Value) -> Self {
        self.query_schema = Some(schema);
        self
    }

    pub fn tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string());
        self
    }

    pub fn get<H, T>(path: &'static str, handler: H) -> Self
    where
        H: Handler<T, S>,
        T: 'static,
    {
        Self::new("GET", path, handler)
    }

    pub fn post<H, T>(path: &'static str, handler: H) -> Self
    where
        H: Handler<T, S>,
        T: 'static,
    {
        Self::new("POST", path, handler)
    }

    pub fn put<H, T>(path: &'static str, handler: H) -> Self
    where
        H: Handler<T, S>,
        T: 'static,
    {
        Self::new("PUT", path, handler)
    }

    pub fn delete<H, T>(path: &'static str, handler: H) -> Self
    where
        H: Handler<T, S>,
        T: 'static,
    {
        Self::new("DELETE", path, handler)
    }

    pub fn patch<H, T>(path: &'static str, handler: H) -> Self
    where
        H: Handler<T, S>,
        T: 'static,
    {
        Self::new("PATCH", path, handler)
    }
}

impl Route<()> {
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
