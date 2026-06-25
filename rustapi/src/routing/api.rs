use axum::Router;
use serde_json::{json, Map};
use crate::docs::{docs_router, finalize_openapi_spec, process_openapi_schema};
use crate::routing::route::{Route, Routable};
use crate::routing::router::RouterBuilder;

/// The main application builder for RustAPI.
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
    /// Create a new RustAPI instance.
    pub fn new() -> Self {
        Self {
            routes: Vec::new(),
            layers: Vec::new(),
            title: "RustAPI".to_string(),
            version: "0.1.0".to_string(),
        }
    }

    /// Set the API title for OpenAPI documentation.
    pub fn title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    /// Set the API version for OpenAPI documentation.
    pub fn version(mut self, version: &str) -> Self {
        self.version = version.to_string();
        self
    }

    /// Add a route or router to the API.
    pub fn route<R: Routable<S>>(mut self, routable: R) -> Self {
        routable.add_to_api(&mut self);
        self
    }

    /// Add a layer (middleware) to the router.
    pub fn layer<F>(mut self, f: F) -> Self
    where
        F: FnOnce(Router<S>) -> Router<S> + Send + 'static,
    {
        self.layers.push(Box::new(f));
        self
    }

    /// Build the final axum Router.
    pub fn build_router(self, state: S) -> Router<()> {
        let mut router = Router::<S>::new();
        for layer_fn in self.layers {
            router = layer_fn(router);
        }
        let mut paths = Map::new();
        let mut components_schemas = Map::new();
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
