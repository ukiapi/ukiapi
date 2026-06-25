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

    println!("🚀  Listening on http://{}", addr);
    println!("📄  Docs at     http://{}/docs", addr);

    axum::serve(listener, router.into_make_service())
        .await
        .unwrap();
}


use axum::extract::FromRequest;

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
        Self { method, path, build, request_schema: None, response_schema: None }
    }

    pub fn with_request_schema(mut self, schema: Value) -> Self {
        self.request_schema = Some(schema);
        self
    }

    pub fn with_response_schema(mut self, schema: Value) -> Self {
        self.response_schema = Some(schema);
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
        }
    }
}


fn schema_to_html(schema: &Value, depth: usize) -> String {
    let indent = "  ".repeat(depth);
    match schema.get("type").and_then(|t| t.as_str()) {
        Some("object") => {
            let title = schema.get("title").and_then(|t| t.as_str()).unwrap_or("object");
            let mut html = format!("{}<div class=\"schema-object\">\n", indent);
            html.push_str(&format!("{}  <strong>{}</strong> {{", indent, title));
            if let Some(props) = schema.get("properties").and_then(|p| p.as_object()) {
                html.push('\n');
                for (name, prop) in props {
                    let prop_type = prop.get("type").and_then(|t| t.as_str()).unwrap_or("any");
                    let required = schema.get("required")
                        .and_then(|r| r.as_array())
                        .map(|r| r.iter().any(|v| v.as_str() == Some(name)))
                        .unwrap_or(false);
                    let req_mark = if required { " *" } else { "" };
                    html.push_str(&format!(
                        "{}    <span class=\"prop\">{}{}:</span> <span class=\"type\">{}</span>\n",
                        indent, name, req_mark, prop_type
                    ));

                }
            }
            html.push_str(&format!("{}}}", indent));
            html
        }
        Some("array") => {
            let title = schema.get("title").and_then(|t| t.as_str()).unwrap_or("array");
            let inner = if let Some(items) = schema.get("items") {
                if items.get("$ref").is_some() {
                    let ref_name = items.get("$ref").and_then(|r| r.as_str()).unwrap_or("?");
                    let short = ref_name.rsplit('/').next().unwrap_or(ref_name);
                    format!("<span class=\"type\">{}</span>", short)
                } else {
                    schema_to_html(items, depth + 1)
                }
            } else {
                String::new()
            };
            format!("{}<div class=\"schema-array\"><strong>{}</strong> [{}]</div>", indent, title, inner.trim())
        }
        _ => {
            match schema.get("type").and_then(|t| t.as_str()) {
                Some(t) => format!("{}<span class=\"type\">{}</span>", indent, t),
                None => format!("{}<span class=\"type\">any</span>", indent),
            }
        }
    }
}

fn build_docs_page(openapi: &Value) -> String {
    let mut html = String::from(
        r#"<!DOCTYPE html>
<html>
<head><title>RustAPI Docs</title>
<style>
body{font-family:sans-serif;max-width:900px;margin:40px auto;padding:0 20px;background:#fafafa}
h1{border-bottom:2px solid #333;padding-bottom:10px}
.endpoint{background:#fff;border:1px solid #e0e0e0;border-radius:8px;padding:16px;margin:16px 0}
.endpoint-header{display:flex;align-items:center;gap:12px;margin-bottom:12px}
.method{display:inline-block;padding:4px 10px;border-radius:4px;color:#fff;font-weight:700;font-size:14px}
.method.GET{background:#61affe}
.method.POST{background:#49cc90}
.method.PUT{background:#fca130}
.method.DELETE{background:#f93e3e}
.method.PATCH{background:#50e3c2}
.path{font-family:monospace;font-size:16px;font-weight:600}
.schema-section{margin:8px 0;padding:8px;background:#f5f5f5;border-radius:4px}
.schema-section h4{margin:0 0 8px;font-size:14px;color:#666}
.schema-object,.schema-array{font-family:monospace;font-size:13px;margin:4px 0}
.prop{color:#881391}
.type{color:#1c7cd6}
a{color:#1c7cd6;text-decoration:none}
a:hover{text-decoration:underline}
</style></head>
<body>
<h1>RustAPI</h1>
<p>Auto-generated OpenAPI 3.0 documentation</p>
"#);

    if let Some(paths) = openapi["paths"].as_object() {
        for (path, item) in paths {
            if let Some(methods) = item.as_object() {
                for (method, operation) in methods {
                    html.push_str("<div class=\"endpoint\">");
                    html.push_str(&format!(
                        "<div class=\"endpoint-header\"><span class=\"method {}\">{}</span><span class=\"path\">{}</span></div>",
                        method.to_uppercase(),
                        method.to_uppercase(),
                        path
                    ));

                    if let Some(body) = operation.get("requestBody") {
                        if let Some(content) = body.get("content") {
                            if let Some(json_content) = content.get("application/json") {
                                if let Some(schema) = json_content.get("schema") {
                                    html.push_str("<div class=\"schema-section\"><h4>Request Body</h4>");
                                    html.push_str(&schema_to_html(schema, 0));
                                    html.push_str("</div>");
                                }
                            }
                        }
                    }

                    if let Some(responses) = operation.get("responses") {
                        if let Some(r200) = responses.get("200") {
                            if let Some(content) = r200.get("content") {
                                if let Some(json_content) = content.get("application/json") {
                                    if let Some(schema) = json_content.get("schema") {
                                        html.push_str("<div class=\"schema-section\"><h4>Response</h4>");
                                        html.push_str(&schema_to_html(schema, 0));
                                        html.push_str("</div>");
                                    }
                                }
                            }
                        }
                    }

                    html.push_str("</div>");
                }
            }
        }
    }

    html.push_str("<p><a href=\"/openapi.json\">View OpenAPI JSON</a></p>");
    html.push_str("</body></html>");
    html
}

pub struct RustAPI<S = ()> {
    routes: Vec<Route<S>>,
    layers: Vec<Box<dyn FnOnce(axum::Router<S>) -> axum::Router<S> + Send>>,
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
                    "application/json": { "schema": schema }
                });
            }

            if let Some(schema) = &route.request_schema {
                operation["requestBody"] = json!({
                    "required": true,
                    "content": {
                        "application/json": { "schema": schema }
                    }
                });
            }

            path_item[&method] = operation;
            router = (route.build)(router);
        }

        let openapi_json = json!({
            "openapi": "3.0.0",
            "info": {
                "title": "RustAPI",
                "version": "0.1.0"
            },
            "paths": paths
        });

        let docs_html = build_docs_page(&openapi_json);

        let stateless_docs = axum::Router::new()
            .route(
                "/docs",
                axum::routing::get(|| async move { Html(docs_html) }),
            )
            .route(
                "/openapi.json",
                axum::routing::get(|| async { Json(openapi_json) }),
            );

        router.with_state(state).merge(stateless_docs)
    }
}
