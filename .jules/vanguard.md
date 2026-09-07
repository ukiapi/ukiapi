## 2024-07-22 - Missing Test for Error Obfuscation
**Issue:** `HTTPException` logic for hiding internal server error details (HTTP 500) behind generic messages lacked unit test coverage.
**Learning:** Security-critical features (like error detail obfuscation) in web framework responses must have explicit unit tests to prevent silent regressions during refactoring.
**Prevention:** Always add specific test cases for edge behaviors involving response transformation, ensuring sensitive context is not leaked to the client.
## 2024-07-22 - Missing Test for Multipart Form Fields
**Issue:** `UploadFile` extractor only parsed the very first multipart field and failed if metadata or non-file fields were positioned earlier in the HTTP request payload, which was untested.
**Learning:** `axum::extract::Multipart` iterates over fields sequentially. Extractors wrapping `Multipart` must iterate through all fields (e.g., using `while let Some(field)`) rather than assuming the file is always the first part.
**Prevention:** Always write unit tests for multipart body extraction that simulate varied field ordering (e.g., text fields before file fields) by explicitly constructing realistic boundary-delimited payloads.
## 2024-09-07 - Missing Test for JSON Deserialization Failure
**Issue:** `ValidatedJson` extractor lacked unit test coverage for requests where the payload is completely empty or fails JSON object deserialization.
**Learning:** In Ukiapi's `TestClient`, to intentionally trigger a serde JSON deserialization error for an endpoint expecting a struct/object body, you can pass a generic string literal reference (e.g., `&""`) to `.post()`. This serializes to a valid JSON string but fails deserialization into the expected object.
**Prevention:** Always write unit tests for JSON validation extractors that explicitly cover deserialization/schema mismatch failures, not just value constraint violations.
