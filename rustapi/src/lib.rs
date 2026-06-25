pub use rustapi_macros::{delete, get, model, patch, post, put};
pub use axum::{
    self,
    extract::{Extension, Path},
    response::IntoResponse,
    response::Html,
    Json,
    extract::State,
};
pub use serde::{Deserialize, Serialize};
pub use serde_json::{json, Value};
pub use schemars::JsonSchema;
pub use validator::Validate;
pub use ts_rs;

/// Start the server. Reads `RUSTAPI_HOST` and `RUSTAPI_PORT` from the
/// environment (set automatically by `rustapi run` / `rustapi dev`),
/// falling back to `127.0.0.1:3000`.
///
/// ```rust
/// #[tokio::main]
/// async fn main() {
///     let app = rustapi::routes![AppState, ...].build_router(state);
///     rustapi::serve(app).await;
/// }
/// ```
pub async fn serve(router: axum::Router<()>) {
    let host = std::env::var("RUSTAPI_HOST").unwrap_or_else(|_| "127.0.0.1".into());
    let port = std::env::var("RUSTAPI_PORT").unwrap_or_else(|_| "3000".into());
    let addr = format!("{}:{}", host, port);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap_or_else(|e| {
        eprintln!("error: could not bind to {}: {}", addr, e);
        std::process::exit(1);
    });

    println!("🚀  Listening on  http://{}", addr);
    println!("📄  Swagger UI    http://{}/docs", addr);
    println!("📘  ReDoc         http://{}/redoc", addr);
    println!("🔧  OpenAPI JSON  http://{}/openapi.json", addr);

    axum::serve(listener, router.into_make_service())
        .await
        .unwrap();
}


use axum::extract::{FromRequest, FromRequestParts};
use axum::http::request::Parts;

pub struct Query<T: Validate>(pub T);

impl<T, S> FromRequestParts<S> for Query<T>
where
    T: serde::de::DeserializeOwned + Validate + Send + Sync + 'static,
    S: Send + Sync + 'static,
{
    type Rejection = (axum::http::StatusCode, Json<Value>);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let axum::extract::Query(query) = axum::extract::Query::<T>::from_request_parts(parts, state)
            .await
            .map_err(|e| {
                (
                    axum::http::StatusCode::UNPROCESSABLE_ENTITY,
                    Json(json!({
                        "detail": format!("Invalid query parameters: {}", e),
                        "errors": null
                    })),
                )
            })?;

        query.validate().map_err(|e| {
            (
                axum::http::StatusCode::UNPROCESSABLE_ENTITY,
                Json(json!({
                    "detail": format!("Validation failed: {}", e),
                })),
            )
        })?;
        
        Ok(Query(query))
    }
}

pub struct ValidatedJson<T: Validate>(pub T);

impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: serde::de::DeserializeOwned + Validate + Send + Sync + 'static, 
    S: Send + Sync + 'static, 
{
    type Rejection = (axum::http::StatusCode, Json<Value>);

    async fn from_request(req: axum::extract::Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(body) = Json::<T>::from_request(req, state).await.map_err(|e| {
            (
                axum::http::StatusCode::UNPROCESSABLE_ENTITY,
                Json(json!({
                    "detail": format!("Invalid JSON: {}", e),
                    "errors": null
                })),
            )
        })?;
        body.validate().map_err(|e| {
            (
                axum::http::StatusCode::UNPROCESSABLE_ENTITY,
                Json(json!({
                    "detail": format!("Validation failed: {}", e),
                })),
            )
        })?;
        Ok(ValidatedJson(body))
    }
}

pub fn schema_for<T: JsonSchema>() -> Value {
    serde_json::to_value(schemars::schema_for!(T)).unwrap()
}

#[macro_export]
macro_rules! routes {
    ($state_ty:ty, $($route:expr),+ $(,)?) => {
        {
            let mut __api = $crate::RustAPI::<$state_ty>::new();
            $(__api = __api.route($route);)+
            __api
        }
    };
}

type RouterBuilder<S> = Box<dyn FnOnce(axum::Router<S>) -> axum::Router<S> + Send>;

pub struct Route<S = ()> {
    pub method: &'static str,
    pub path: &'static str,
    pub(crate) build: RouterBuilder<S>,
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

/// Swagger UI — served at /docs
fn build_swagger_page() -> String {
    r#"<!DOCTYPE html>
<html>
<head>
  <title>RustAPI — Swagger UI</title>
  <meta charset="utf-8"/>
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <link rel="stylesheet" href="https://unpkg.com/swagger-ui-dist@5/swagger-ui.css">
</head>
<body>
<div id="swagger-ui"></div>
<script src="https://unpkg.com/swagger-ui-dist@5/swagger-ui-bundle.js"></script>
<script>
  window.onload = () => {
    SwaggerUIBundle({
      url: '/openapi.json',
      dom_id: '#swagger-ui',
      presets: [SwaggerUIBundle.presets.apis, SwaggerUIBundle.SwaggerUIStandalonePreset],
      layout: 'BaseLayout',
      deepLinking: true,
      tryItOutEnabled: true,
    });
  };
</script>
</body>
</html>
"#.to_string()
}

/// ReDoc — served at /redoc
fn build_redoc_page() -> String {
    r#"<!DOCTYPE html>
<html>
<head>
  <title>RustAPI — ReDoc</title>
  <meta charset="utf-8"/>
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <link href="https://fonts.googleapis.com/css?family=Montserrat:300,400,700|Roboto:300,400,700" rel="stylesheet">
  <style>body { margin: 0; padding: 0; }</style>
</head>
<body>
  <redoc spec-url='/openapi.json'></redoc>
  <script src="https://cdn.redoc.ly/redoc/latest/bundles/redoc.standalone.js"></script>
</body>
</html>
"#.to_string()
}


pub struct RustAPI<S = ()> {
    routes: Vec<Route<S>>,
    layers: Vec<RouterBuilder<S>>,
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
        Self { routes: Vec::new(), layers: Vec::new() }
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

            let mut process_schema = |schema: &Value| -> Value {
                let mut schema_clone = schema.clone();
                if let Some(obj) = schema_clone.as_object_mut() {
                    if let Some(defs) = obj.remove("definitions") {
                        if let Some(defs_obj) = defs.as_object() {
                            for (k, v) in defs_obj {
                                components_schemas.insert(k.clone(), v.clone());
                            }
                        }
                    }
                }
                schema_clone
            };

            if let Some(schema) = &route.response_schema {
                operation["responses"]["200"]["content"] = json!({
                    "application/json": { "schema": process_schema(schema) }
                });
            }

            if let Some(schema) = &route.query_schema {
                let schema = process_schema(schema);
                if let Some(props) = schema.get("properties").and_then(|p| p.as_object()) {
                    let required_fields = schema.get("required")
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
                        "application/json": { "schema": process_schema(schema) }
                    }
                });
            }

            path_item[&method] = operation;
            router = (route.build)(router);
        }

        let mut openapi_json = json!({
            "openapi": "3.0.0",
            "info": {
                "title": "RustAPI",
                "version": "0.1.0"
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
