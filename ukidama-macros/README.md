# ukidama-macros

Procedural macros for the Ukidama framework.

## 🚀 Macros

### `#[get("path")]`, `#[post("path")]`, etc.
Attribute macros to define API endpoints. They generate a helper function (e.g., `my_handler_route()`) that returns a `Route` object used by the `Ukidama` builder.
They also automatically infer request/response schemas for OpenAPI documentation.

### `#[model]`
A shortcut macro for data models. It derives:
- `Serialize`, `Deserialize` (Serde)
- `JsonSchema` (Schemars)
- `Validate` (Validator)
- `TS` (ts-rs) for TypeScript bindings
- `Clone`
