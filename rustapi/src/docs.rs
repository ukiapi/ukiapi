use serde_json::Value;

/// Swagger UI — served at /docs
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

/// ReDoc — served at /redoc
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
