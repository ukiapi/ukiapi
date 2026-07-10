# About UkiApi

## Overview

UkiApi is a high-performance web framework for Rust, inspired by FastAPI. It provides automatic documentation, type-safe routing, and seamless validation.

## Goals

1. **Developer Experience**: Minimize boilerplate while maximizing productivity
2. **Performance**: Leverage Rust's zero-cost abstractions
3. **Type Safety**: Catch errors at compile time
4. **Documentation**: Auto-generate API docs from code
5. **Extensibility**: Easy to add custom middleware and features

## Architecture

```
ukiapi/
├── src/
│   ├── routing/       # Router, middleware, route discovery
│   ├── extract/       # Request extractors
│   ├── response/      # Response types
│   ├── auth/          # JWT authentication
│   ├── ws/            # WebSocket support
│   └── ...
ukiapi-macros/         # Proc macros
uki/                   # CLI tool
```

## Technology Stack

- **Axum** - Web framework
- **Tokio** - Async runtime
- **Tower** - Middleware
- **Serde** - Serialization
- **Schemars** - JSON Schema
- **Validator** - Data validation
- **SQLx** - Database (optional)

## Project Status

**Version:** 0.0.1

UkiApi is under active development. The core API is stable, but some features may change.

## License

MIT License

## Links

- [GitHub](https://github.com/ukiapi/ukiapi)
- [crates.io](https://crates.io/crates/ukiapi) (coming soon)
- [Documentation](https://ukiapi.github.io/ukiapi)

## Acknowledgments

- [FastAPI](https://fastapi.tiangolo.com/) - Inspiration for the API design
- [Axum](https://github.com/tokio-rs/axum) - The foundation of UkiApi
- [Tokio](https://tokio.rs/) - The async runtime
