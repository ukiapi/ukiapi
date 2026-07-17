## 2024-07-25 - Prevent Information Exposure in Dependency Injection
**Vulnerability:** Internal error details and `type_name::<D>()` were being passed directly to the client when a scoped dependency failed to resolve.
**Learning:** The `Into<HTTPException>` implementation for `ScopedDiError` was returning the raw `msg` generated from `format!("{:?}", err)` in a 500 error response.
**Prevention:** Map internal errors to generic error messages like 'Internal server error' before responding to the client to avoid leaking infrastructure details.
