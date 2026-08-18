## 2024-07-22 - Missing Test for Error Obfuscation
**Issue:** `HTTPException` logic for hiding internal server error details (HTTP 500) behind generic messages lacked unit test coverage.
**Learning:** Security-critical features (like error detail obfuscation) in web framework responses must have explicit unit tests to prevent silent regressions during refactoring.
**Prevention:** Always add specific test cases for edge behaviors involving response transformation, ensuring sensitive context is not leaked to the client.
## 2024-07-22 - Missing Test for Multipart Form Fields
**Issue:** `UploadFile` extractor only parsed the very first multipart field and failed if metadata or non-file fields were positioned earlier in the HTTP request payload, which was untested.
**Learning:** `axum::extract::Multipart` iterates over fields sequentially. Extractors wrapping `Multipart` must iterate through all fields (e.g., using `while let Some(field)`) rather than assuming the file is always the first part.
**Prevention:** Always write unit tests for multipart body extraction that simulate varied field ordering (e.g., text fields before file fields) by explicitly constructing realistic boundary-delimited payloads.
## 2024-07-22 - Missing Test for Extractor Deserialization Failures
**Issue:** `ValidatedJson` and `Query` extractors were only tested for successful requests, schema validation failures, and missing fields. Complete deserialization failures (e.g., passing a generic string when a JSON object is expected, or passing unparseable strings for typed query fields) were entirely untested.
**Learning:** Extractors that combine raw serde deserialization and secondary schema validation (like `validator::Validate`) have two distinct failure domains. Both must be explicitly tested. Passing invalid types (e.g., strings instead of objects) triggers the `serde_json` error path before the validation path is ever reached.
**Prevention:** In API test suites for complex extractors, explicitly write unit tests that fail structural deserialization by passing incompatible primitives or malformed strings, verifying that the `FromRequest` wrapper correctly catches the underlying library error and returns the intended HTTP status.
