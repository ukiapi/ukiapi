## 2024-05-24 - Prevent Info Leakage in HTTPException
**Vulnerability:** Internal error details and dependency injection contexts were being exposed to clients through `HTTPException`'s JSON response for HTTP 500 errors.
**Learning:** The `IntoResponse` implementation for `HTTPException` directly serialized `self.detail` regardless of the status code, failing to distinguish between safe client-facing errors (4xx) and sensitive internal errors (5xx).
**Prevention:** Always mask internal error details in global exception handlers by returning a generic "Internal Server Error" message when the status code is a 5xx server error.
## 2025-01-20 - Prevent Path Traversal in Upload Handler
**Vulnerability:** Found a Path Traversal vulnerability in the `/upload` endpoint of `example/src/routes.rs`. The code blindly extracted `file.filename` and appended it to a temporary directory without checking for parent directory sequences (`../`).
**Learning:** `UploadFile` filenames sent by clients are untrusted input. Blindly joining a path can overwrite arbitrary files if the filename contains traversal characters (e.g. `../../../etc/passwd`).
**Prevention:** Sanitize the filename by using `std::path::Path::new(...).file_name()` to extract only the final file component, ignoring any directory paths.
## 2024-05-18 - Restricting CORS
**Vulnerability:** Weak CORS settings.
**Learning:** `CorsLayer::permissive()` allows cross-origin requests from anywhere, which is a security risk.
**Prevention:** Restrict origins for CORS.
## 2024-05-18 - Hardcoded secret in development
**Vulnerability:** Weak default secret.
**Learning:** `unwrap_or_else(|_| "development_secret".to_string())` could accidentally be deployed to production. Should enforce secret to be present or at least log heavily.
**Prevention:** Avoid default fallback for secrets, require them to be configured or fail fast.
## 2024-05-18 - XSS possibility
**Vulnerability:** Weak CORS configuration.
**Learning:** Returning `CorsLayer::permissive()` allows cross-site origin access to all routes, making it vulnerable to CSRF and unauthenticated requests.
**Prevention:** Avoid permissive layer, specify specific origins and methods instead.
