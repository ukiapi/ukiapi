## 2024-05-24 - Prevent Info Leakage in HTTPException
**Vulnerability:** Internal error details and dependency injection contexts were being exposed to clients through `HTTPException`'s JSON response for HTTP 500 errors.
**Learning:** The `IntoResponse` implementation for `HTTPException` directly serialized `self.detail` regardless of the status code, failing to distinguish between safe client-facing errors (4xx) and sensitive internal errors (5xx).
**Prevention:** Always mask internal error details in global exception handlers by returning a generic "Internal Server Error" message when the status code is a 5xx server error.
