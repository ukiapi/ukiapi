# Introduction to UkiApi

UkiApi is a Rust web framework inspired by FastAPI, designed for building modern APIs with minimal boilerplate.

## Philosophy

- **Developer Experience**: Write less code, get more done
- **Type Safety**: Catch errors at compile time
- **Performance**: Built on Axum and Tokio for async efficiency
- **Documentation**: Auto-generated API docs from your code

## Why UkiApi?

| Feature | UkiApi | Traditional |
|---------|--------|-------------|
| Route Definition | `#[get("/path")]` | Manual router setup |
| Validation | Derive macros | Manual checks |
| API Docs | Automatic | Manual/OpenAPI |
| Type Safety | Full | Partial |

## Architecture

```
ukiapi/          # Core library
├── src/
│   ├── routing/     # Router and middleware
│   ├── extract/     # Request extractors
│   ├── response/    # Response types
│   └── ...
ukiapi-macros/   # Proc macros
uki/             # CLI tool
```

## Supported Features

- RESTful routing with `#[get]`, `#[post]`, `#[put]`, `#[patch]`, `#[delete]`
- WebSocket support with `#[websocket]`
- Request validation with `validator`
- Automatic Swagger UI and ReDoc
- JWT authentication helpers
- Background task scheduling
- Static file serving
- Multipart upload handling
