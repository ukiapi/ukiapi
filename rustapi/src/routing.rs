use crate::docs::{docs_router, finalize_openapi_spec, process_openapi_schema};
use serde_json::{json, Value};

pub type RouteAdder<S> = Box<dyn FnOnce(axum::Router<S>, &str) -> axum::Router<S> + Send>;
pub type RouterBuilder<S> = Box<dyn FnOnce(axum::Router<S>) -> axum::Router<S> + Send>;

pub trait Routable<S> {
    fn add_to_api(self, api: &mut RustAPI<S>);
}

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
    fn add_to_api(self, api: &mut RustAPI<S>) {
        api.routes.push(self);
    }
}

impl<S> Route<S>
where
    S: Clone + Send + Sync + 'static,
{
    fn new<H, T>(method: &'static str, path: &'static str, handler: H) -> Self
    where
        H: axum::handler::Handler<T, S>,
        T: 'static,
    {
        let adder: RouteAdder<S> = match method {
            "GET" => Box::new(move |router, path| router.route(path, axum::routing::get(handler))),
            "POST" => {
                Box::new(move |router, path| router.route(path, axum::routing::post(handler)))
            }
            "PUT" => Box::new(move |router, path| router.route(path, axum::routing::put(handler))),
            "DELETE" => {
                Box::new(move |router, path| router.route(path, axum::routing::delete(handler)))
            }
            "PATCH" => {
                Box::new(move |router, path| router.route(path, axum::routing::patch(handler)))
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
        H: axum::handler::Handler<T, S>,
        T: 'static,
    {
        Self::new("GET", path, handler)
    }

    pub fn post<H, T>(path: &'static str, handler: H) -> Self
    where
        H: axum::handler::Handler<T, S>,
        T: 'static,
    {
        Self::new("POST", path, handler)
    }

    pub fn put<H, T>(path: &'static str, handler: H) -> Self
    where
        H: axum::handler::Handler<T, S>,
        T: 'static,
    {
        Self::new("PUT", path, handler)
    }

    pub fn delete<H, T>(path: &'static str, handler: H) -> Self
    where
        H: axum::handler::Handler<T, S>,
        T: 'static,
    {
        Self::new("DELETE", path, handler)
    }

    pub fn patch<H, T>(path: &'static str, handler: H) -> Self
    where
        H: axum::handler::Handler<T, S>,
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
        let new_adder: RouteAdder<NewS> = Box::new(move |router: axum::Router<NewS>, path| {
            let stateless_router = axum::Router::<()>::new();
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

pub struct APIRouter<S = ()> {
    pub prefix: String,
    pub tags: Vec<String>,
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
    pub fn new() -> Self {
        Self {
            prefix: String::new(),
            tags: Vec::new(),
            routes: Vec::new(),
        }
    }

    pub fn prefix(mut self, prefix: &str) -> Self {
        self.prefix = prefix.to_string();
        self
    }

    pub fn tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string());
        self
    }

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

pub struct RustAPI<S = ()> {
    pub(crate) routes: Vec<Route<S>>,
    layers: Vec<RouterBuilder<S>>,
    title: String,
    version: String,
}

impl<S> Default for RustAPI<S>
where
    S: Clone + Send + Sync + 'static,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<S> RustAPI<S>
where
    S: Clone + Send + Sync + 'static,
{
    pub fn new() -> Self {
        Self {
            routes: Vec::new(),
            layers: Vec::new(),
            title: "RustAPI".to_string(),
            version: "0.1.0".to_string(),
        }
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    pub fn version(mut self, version: &str) -> Self {
        self.version = version.to_string();
        self
    }

    pub fn route<R: Routable<S>>(mut self, routable: R) -> Self {
        routable.add_to_api(&mut self);
        self
    }

    pub fn layer<F>(mut self, f: F) -> Self
    where
        F: FnOnce(axum::Router<S>) -> axum::Router<S> + Send + 'static,
    {
        self.layers.push(Box::new(f));
        self
    }

    pub fn build_router(self, state: S) -> axum::Router<()> {
        let mut router = axum::Router::<S>::new();
        for layer_fn in self.layers {
            router = layer_fn(router);
        }
        let mut paths = serde_json::Map::new();
        let mut components_schemas = serde_json::Map::new();
        for route in self.routes {
            let path_key = &route.path;
            let method = route.method.to_lowercase();
            let path_item = paths
                .entry(path_key.to_string())
                .or_insert_with(|| json!({}));
            let mut operation = json!({"responses": {"200": {"description": "OK"}}});
            if !route.tags.is_empty() {
                operation["tags"] = json!(route.tags);
            }
            if let Some(schema) = &route.response_schema {
                operation["responses"]["200"]["content"] = json!({"application/json": {"schema": process_openapi_schema(schema, &mut components_schemas)}});
            }
            if let Some(schema) = &route.query_schema {
                let schema = process_openapi_schema(schema, &mut components_schemas);
                if let Some(props) = schema.get("properties").and_then(|p| p.as_object()) {
                    let req = schema
                        .get("required")
                        .and_then(|r| r.as_array())
                        .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect::<Vec<_>>())
                        .unwrap_or_default();
                    let mut parameters = Vec::new();
                    for (name, p_schema) in props {
                        parameters.push(json!({"name": name, "in": "query", "required": req.contains(&name.as_str()), "schema": p_schema}));
                    }
                    operation["parameters"] = json!(parameters);
                }
            }
            if let Some(schema) = &route.request_schema {
                operation["requestBody"] = json!({"required": true, "content": {"application/json": {"schema": process_openapi_schema(schema, &mut components_schemas)}}});
            }
            path_item[&method] = operation;
            router = (route.adder)(router, &route.path);
        }
        let openapi_json =
            finalize_openapi_spec(self.title, self.version, paths, components_schemas);
        router.with_state(state).merge(docs_router(openapi_json))
    }
}
