## 2024-06-26 - Hardcoded internal secret in examples

**Vulnerability:** A hardcoded `internal_secret` ("secret123") was present in the `create_item` endpoint in `example/src/routes.rs`.
**Learning:** Example projects often embed secrets directly to simplify setup for new users. However, these can inadvertently be deployed or copied directly into real projects by users assuming it's the standard practice.
**Prevention:** Use environment variables (e.g. `std::env::var("INTERNAL_SECRET")`) combined with `unwrap_or_else` or similar constructs to provide a seamless development fallback without embedding production secrets in the codebase itself. Ensure all scaffolded or example code follows production security practices.
## 2026-06-26 - [Background Tasks and File Uploads]
**Vulnerability:** N/A.
**Learning:** Background tasks can be implemented by storing them in request extensions and spawning them after the response is sent. File uploads can be simplified by providing a wrapper around Axum's Multipart extractor.
**Prevention:** N/A.

## 2024-05-15 - [Path Traversal via Unsanitized Multipart Filename]
**Vulnerability:** The `UploadFile` extractor in `rustapi/src/upload.rs` directly exposed the client-provided `filename` parameter without any sanitization. This allowed attackers to provide paths containing directory traversal sequences (e.g., `../../../etc/passwd` or `..\..\..\Windows\System32`), which developers might inadvertently use directly in file I/O operations (as seen in the example app's `save` method usage).
**Learning:** Framework-level extractors that parse untrusted user input (especially for file operations) must perform defense-in-depth sanitization at the boundary. Trusting the developer to sanitize the filename downstream creates an implicit architectural security gap, leading to widespread vulnerabilities in apps built with the framework.
**Prevention:** Always sanitize paths extracted from external sources (like multipart form data) at the framework level before passing them to application logic. Using `s.replace('\\', "/").rsplit('/').next()` ensures only the base filename is exposed, preventing directory traversal.
