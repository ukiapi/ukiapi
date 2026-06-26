use crate::response::{Html, Json};
use crate::routing::methods;
use crate::routing::Router;
use serde_json::{json, Value};

/// Build the Swagger UI HTML page.
pub fn build_swagger_page() -> String {
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
"#
    .to_string()
}

/// Build the ReDoc HTML page.
pub fn build_redoc_page() -> String {
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

/// Process a JSON schema to move definitions to components/schemas.
pub fn process_openapi_schema(
    schema: &Value,
    components_schemas: &mut serde_json::Map<String, Value>,
) -> Value {
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
}

/// Finalize the OpenAPI specification by merging paths and schemas.
pub fn finalize_openapi_spec(
    title: String,
    version: String,
    paths: serde_json::Map<String, Value>,
    components_schemas: serde_json::Map<String, Value>,
) -> Value {
    let mut openapi_json = json!({
        "openapi": "3.0.0",
        "info": {
            "title": title,
            "version": version
        },
        "paths": paths
    });
    if !components_schemas.is_empty() {
        openapi_json["components"] = json!({ "schemas": components_schemas });
    }
    let openapi_str = serde_json::to_string(&openapi_json)
        .unwrap()
        .replace("#/definitions/", "#/components/schemas/");
    serde_json::from_str(&openapi_str).unwrap()
}

/// Create a router to serve OpenAPI JSON and documentation UI.
pub fn docs_router(openapi_json: Value) -> Router {
    let swagger_html = build_swagger_page();
    let redoc_html = build_redoc_page();

    Router::new()
        .route("/docs", methods::get(|| async move { Html(swagger_html) }))
        .route("/redoc", methods::get(|| async move { Html(redoc_html) }))
        .route(
            "/openapi.json",
            methods::get(|| async { Json(openapi_json) }),
        )
}
