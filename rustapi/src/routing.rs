use crate::docs::{build_redoc_page, build_swagger_page, process_openapi_schema};
use axum::response::{Html, Json};
use serde_json::{json, Value};

pub type RouterBuilder<S> = Box<dyn FnOnce(axum::Router<S>) -> axum::Router<S> + Send>;

pub struct Route<S = ()> {
    pub method: &'static str,
    pub path: &'static str,
    pub build: RouterBuilder<S>,
    pub request_schema: Option<Value>,
    pub response_schema: Option<Value>,
    pub query_schema: Option<Value>,
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
        let build: RouterBuilder<S> = match method {
            "GET" => Box::new(move |router| router.route(path, axum::routing::get(handler))),
            "POST" => Box::new(move |router| router.route(path, axum::routing::post(handler))),
            "PUT" => Box::new(move |router| router.route(path, axum::routing::put(handler))),
            "DELETE" => Box::new(move |router| router.route(path, axum::routing::delete(handler))),
            "PATCH" => Box::new(move |router| router.route(path, axum::routing::patch(handler))),
            _ => unreachable!(),
        };
        Self {
            method,
            path,
            build,
            request_schema: None,
            response_schema: None,
            query_schema: None,
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
        let old_build = self.build;
        let new_build: RouterBuilder<NewS> = Box::new(move |router: axum::Router<NewS>| {
            let stateless_router = axum::Router::<()>::new();
            let built_stateless_router = old_build(stateless_router);
            router.merge(built_stateless_router.with_state(()))
        });

        Route {
            method: self.method,
            path: self.path,
            build: new_build,
            request_schema: self.request_schema,
            response_schema: self.response_schema,
            query_schema: self.query_schema,
        }
    }
}

pub struct RustAPI<S = ()> {
    routes: Vec<Route<S>>,
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

    pub fn route(mut self, route: Route<S>) -> Self {
        self.routes.push(route);
        self
    }

    pub fn layer<F>(mut self, f: F) -> Self
    where
        F: FnOnce(axum::Router<S>) -> axum::Router<S> + Send + 'static,
    {
        self.layers.push(Box::new(f));
        self
    }

    pub fn with_state(self, state: S) -> Self {
        self.layer(|router| router.layer(axum::Extension(state)))
    }

    pub fn build_router(self, state: S) -> axum::Router<()> {
        let mut router = axum::Router::<S>::new();

        for layer_fn in self.layers {
            router = layer_fn(router);
        }
        let mut paths = serde_json::Map::new();
        let mut components_schemas = serde_json::Map::new();

        for route in self.routes {
            let path_key = route.path;
            let method = route.method.to_lowercase();
            let path_item = paths
                .entry(path_key.to_string())
                .or_insert_with(|| json!({}));

            let mut operation = json!({
                "responses": { "200": { "description": "OK" } }
            });

            if let Some(schema) = &route.response_schema {
                operation["responses"]["200"]["content"] = json!({
                    "application/json": { "schema": process_openapi_schema(schema, &mut components_schemas) }
                });
            }

            if let Some(schema) = &route.query_schema {
                let schema = process_openapi_schema(schema, &mut components_schemas);
                if let Some(props) = schema.get("properties").and_then(|p| p.as_object()) {
                    let required_fields = schema
                        .get("required")
                        .and_then(|r| r.as_array())
                        .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect::<Vec<_>>())
                        .unwrap_or_default();

                    let mut parameters = Vec::new();
                    for (param_name, param_schema) in props {
                        let is_required = required_fields.contains(&param_name.as_str());
                        parameters.push(json!({
                            "name": param_name,
                            "in": "query",
                            "required": is_required,
                            "schema": param_schema
                        }));
                    }
                    operation["parameters"] = json!(parameters);
                }
            }

            if let Some(schema) = &route.request_schema {
                operation["requestBody"] = json!({
                    "required": true,
                    "content": {
                        "application/json": { "schema": process_openapi_schema(schema, &mut components_schemas) }
                    }
                });
            }

            path_item[&method] = operation;
            router = (route.build)(router);
        }

        let mut openapi_json = json!({
            "openapi": "3.0.0",
            "info": {
                "title": self.title,
                "version": self.version
            },
            "paths": paths
        });

        if !components_schemas.is_empty() {
            openapi_json["components"] = json!({
                "schemas": components_schemas
            });
        }

        // Fix all references from #/definitions/ to #/components/schemas/
        let mut openapi_str = serde_json::to_string(&openapi_json).unwrap();
        openapi_str = openapi_str.replace("#/definitions/", "#/components/schemas/");
        openapi_json = serde_json::from_str(&openapi_str).unwrap();

        let swagger_html = build_swagger_page();
        let redoc_html = build_redoc_page();

        let stateless_docs = axum::Router::new()
            .route(
                "/docs",
                axum::routing::get(|| async move { Html(swagger_html) }),
            )
            .route(
                "/redoc",
                axum::routing::get(|| async move { Html(redoc_html) }),
            )
            .route(
                "/openapi.json",
                axum::routing::get(|| async { Json(openapi_json) }),
            );

        router.with_state(state).merge(stateless_docs)
    }
}
