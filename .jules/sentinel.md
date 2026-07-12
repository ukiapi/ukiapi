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
