use crate::docs::{docs_router, finalize_openapi_spec, process_openapi_schema};
use crate::lifecycle::LifecycleHandler;
use crate::mount::Mount;
use crate::route::Route;
use serde_json::{json};
use std::future::Future;

pub type RouteAdder<S> = Box<dyn FnOnce(axum::Router<S>, &str) -> axum::Router<S> + Send>;
pub type RouterBuilder<S> = Box<dyn FnOnce(axum::Router<S>) -> axum::Router<S> + Send>;

pub trait Routable<S> {
    fn add_to_api(self, api: &mut RustAPI<S>);
}

pub struct APIRouter<S = ()> {
    pub prefix: String,
    pub tags: Vec<String>,
    pub routes: Vec<Route<S>>,
    pub mounts: Vec<Mount<S>>,
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
            mounts: Vec::new(),
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

    pub fn mount(mut self, path: &str, directory: &str) -> Self {
        self.mounts.push(Mount {
            path: format!("{}{}", self.prefix, path),
            directory: directory.to_string(),
            _phantom: std::marker::PhantomData,
        });
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
        for mut mount in temp_api.mounts {
            mount.path = format!("{}{}", self.prefix, mount.path);
            self.mounts.push(mount);
        }
        self
    }
}

impl<S> Routable<S> for APIRouter<S> {
    fn add_to_api(self, api: &mut RustAPI<S>) {
        api.routes.extend(self.routes);
        api.mounts.extend(self.mounts);
    }
}

pub struct RustAPI<S = ()> {
    pub(crate) routes: Vec<Route<S>>,
    pub(crate) mounts: Vec<Mount<S>>,
    layers: Vec<RouterBuilder<S>>,
    startup_handlers: Vec<LifecycleHandler<S>>,
    shutdown_handlers: Vec<LifecycleHandler<S>>,
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
            mounts: Vec::new(),
            layers: Vec::new(),
            startup_handlers: Vec::new(),
            shutdown_handlers: Vec::new(),
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

    pub fn on_startup<F, Fut>(mut self, handler: F) -> Self
    where
        F: FnOnce(S) -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        self.startup_handlers.push(Box::new(|state| Box::pin(handler(state))));
        self
    }

    pub fn on_shutdown<F, Fut>(mut self, handler: F) -> Self
    where
        F: FnOnce(S) -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        self.shutdown_handlers.push(Box::new(|state| Box::pin(handler(state))));
        self
    }

    pub fn mount(mut self, path: &str, directory: &str) -> Self {
        self.mounts.push(Mount {
            path: path.to_string(),
            directory: directory.to_string(),
            _phantom: std::marker::PhantomData,
        });
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
        for mount in self.mounts {
            router = router.nest_service(&mount.path, tower_http::services::ServeDir::new(&mount.directory));
        }
        let openapi_json =
            finalize_openapi_spec(self.title, self.version, paths, components_schemas);
        router.with_state(state).merge(docs_router(openapi_json))
    }

    pub async fn serve(mut self, state: S) {
        let host = std::env::var("RUSTAPI_HOST").unwrap_or_else(|_| "127.0.0.1".into());
        let port = std::env::var("RUSTAPI_PORT").unwrap_or_else(|_| "3000".into());
        let addr = format!("{}:{}", host, port);

        let startup_handlers = std::mem::take(&mut self.startup_handlers);
        for handler in startup_handlers {
            handler(state.clone()).await;
        }

        let shutdown_handlers = std::mem::take(&mut self.shutdown_handlers);
        let app = self.build_router(state.clone());

        let listener = tokio::net::TcpListener::bind(&addr)
            .await
            .unwrap_or_else(|e| {
                eprintln!("error: could not bind to {}: {}", addr, e);
                std::process::exit(1);
            });

        println!("🚀  Listening on  http://{}", addr);
        println!("📄  Swagger UI    http://{}/docs", addr);
        println!("📘  ReDoc         http://{}/redoc", addr);
        println!("🔧  OpenAPI JSON  http://{}/openapi.json", addr);

        let shutdown_state = state.clone();

        axum::serve(listener, app.into_make_service())
            .with_graceful_shutdown(async move {
                tokio::signal::ctrl_c()
                    .await
                    .expect("failed to install CTRL+C handler");

                for handler in shutdown_handlers {
                    handler(shutdown_state.clone()).await;
                }
            })
            .await
            .unwrap();
    }
}
