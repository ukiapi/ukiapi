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
        F: FnOnce(rustapi::Router<S>) -> rustapi::Router<S> + Send + 'static,
    {
        self.layers.push(Box::new(f));
        self
    }

    pub fn with_state(self, state: S) -> Self {
        self.layer(|router| router.layer(rustapi::extract::Extension(state)))
    }

    pub fn build_router(self, state: S) -> rustapi::Router<()> {
        let mut router = rustapi::Router::<S>::new();

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

            let mut operation = json!({
                "responses": { "200": { "description": "OK" } }
            });

            if !route.tags.is_empty() {
                operation["tags"] = json!(route.tags);
            }

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
            router = (route.adder)(router, &route.path);
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

        let stateless_docs = rustapi::Router::new()
            .route(
                "/docs",
                rustapi::routing::methods::get(|| async move { rustapi::response::Html(swagger_html) }),
            )
            .route(
                "/redoc",
                rustapi::routing::methods::get(|| async move { rustapi::response::Html(redoc_html) }),
            )
            .route(
                "/openapi.json",
                rustapi::routing::methods::get(|| async { rustapi::response::Json(openapi_json) }),
            );

        router.with_state(state).merge(stateless_docs)
    }
}
