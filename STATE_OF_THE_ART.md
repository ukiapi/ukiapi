# State of the Art: Ukidama vs. Modern Rust Web Frameworks

## Introduction to Ukidama

Ukidama is a web framework built on top of Axum, designed to offer a developer experience inspired by Python's FastAPI. Its primary goal is to provide highly ergonomic API development in Rust, featuring automatic OpenAPI documentation, robust data validation, and a clear structure for building RESTful services. Ukidama leverages Rust's strong type system and performance characteristics while aiming to simplify common web development patterns.

## Comparison: Ukidama vs. Key Rust Web Frameworks

### 1. Ukidama vs. Axum

**Axum** is a web application framework that focuses on ergonomics and modularity, built by the Tokio team. It leverages Tokio's ecosystem and is known for its flexible design, allowing developers to pick and choose components.

| Feature / Aspect             | Axum                                      | Ukidama                                                         |
| :--------------------------- | :---------------------------------------- | :-------------------------------------------------------------- |
| **Foundation**               | Tokio, Tower, Hyper                       | Axum                                                            |
| **Philosophy**               | Minimalist, composable, "batteries not included" | FastAPI-inspired ergonomics, opinionated, "batteries included"  |
| **Routing**                  | Explicit `Router::route` calls            | Macros (`#[get]`, `#[post]`, etc.) for declarative routing      |
| **OpenAPI/Swagger**          | Requires manual integration (e.g., `utoipa`) | Automatic generation integrated via macros and `schemars`       |
| **Data Validation**          | Requires manual integration (e.g., `validator`, `serde`) | Integrated via `model!` macro, `Query`, `ValidatedJson`          |
| **Dependency Injection**     | `axum::Extension`, custom extractors      | Aims for more explicit/flexible `Depends`-like system           |
| **Error Handling**           | `Result`, `IntoResponse` for rejections | Aims for `HTTPException`-like structured error handling         |
| **Project Structure**        | Very flexible, user-defined               | Promotes modularity with `APIRouter`-like class for organization |
| **Learning Curve**           | Moderate (requires understanding Tower)   | Lower for Python/FastAPI users, still Rust-idiomatic            |
| **Boilerplate**              | Higher for basic API setup                | Significantly reduced for common API patterns                   |
| **Current Status**           | Mature, widely adopted                    | Under active development, aiming for feature parity with FastAPI core |

**Key Differentiator:** Ukidama builds on Axum's solid foundation but adds a layer of opinionated macros and integrated utilities to drastically reduce boilerplate and streamline common API development tasks, especially for projects valuing auto-documentation and validation.

### 2. Ukidama vs. Actix-web

**Actix-web** is one of the most popular and historically highest-performing Rust web frameworks. It's a "full-featured" framework with its own actor system, making it powerful but sometimes perceived as more complex than minimalist alternatives.

| Feature / Aspect             | Actix-web                                 | Ukidama                                                            |
| :--------------------------- | :---------------------------------------- | :----------------------------------------------------------------- |
| **Foundation**               | Actix (actor system), Tokio               | Axum, Tokio, Tower, Hyper                                          |
| **Philosophy**               | Performance-focused, actor-based, comprehensive | FastAPI-inspired ergonomics, developer experience, type-safety     |
| **Routing**                  | Macros (`#[get("/path")]`), service configurators | Macros (`#[get]`), `APIRouter`-like class, OpenAPI integrated      |
| **OpenAPI/Swagger**          | Requires manual integration (e.g., `utoipa`) | Automatic generation integrated via macros and `schemars`          |
| **Data Validation**          | External crates (`serde`, `validator`)    | Integrated via `model!` macro, `Query`, `ValidatedJson`             |
| **Dependency Injection**     | Application state, extractors             | Aims for more explicit/flexible `Depends`-like system              |
| **Error Handling**           | Custom `ResponseError` trait              | Aims for `HTTPException`-like structured error handling            |
| **Performance**              | Historically very high, top-tier          | Inherits Axum's strong performance, highly competitive (benchmarks vary) |
| **Ecosystem**                | Rich, mature actor model ecosystem        | Leverages Axum/Tokio ecosystem, growing tools                     |
| **Learning Curve**           | Moderate to High (actor model concepts)   | Moderate (Rust-idiomatic with FastAPI patterns)                   |

**Key Differentiator:** While both aim for high performance, Actix-web's actor model can introduce a different paradigm. Ukidama focuses on a more direct, function-based routing approach with deep integration of OpenAPI and validation, appealing to developers who prioritize DX and auto-documentation without compromising on Rust's benefits.

## Conclusion

Ukidama carves a niche by combining the robust, performant foundation of Axum with the developer-friendly patterns popularized by FastAPI. For projects where automatic API documentation, schema validation, and a batteries-included approach to common web development tasks are paramount, Ukidama aims to provide a highly productive and type-safe environment, making the transition for developers familiar with frameworks like FastAPI smoother, while still benefiting from the Rust ecosystem's strengths.
