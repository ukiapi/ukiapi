## 2024-07-22 - Missing Test for Error Obfuscation
**Issue:** `HTTPException` logic for hiding internal server error details (HTTP 500) behind generic messages lacked unit test coverage.
**Learning:** Security-critical features (like error detail obfuscation) in web framework responses must have explicit unit tests to prevent silent regressions during refactoring.
**Prevention:** Always add specific test cases for edge behaviors involving response transformation, ensuring sensitive context is not leaked to the client.
## 2024-07-22 - Missing Test for Multipart Form Fields
**Issue:** `UploadFile` extractor only parsed the very first multipart field and failed if metadata or non-file fields were positioned earlier in the HTTP request payload, which was untested.
**Learning:** `axum::extract::Multipart` iterates over fields sequentially. Extractors wrapping `Multipart` must iterate through all fields (e.g., using `while let Some(field)`) rather than assuming the file is always the first part.
**Prevention:** Always write unit tests for multipart body extraction that simulate varied field ordering (e.g., text fields before file fields) by explicitly constructing realistic boundary-delimited payloads.
## 2024-08-05 - Missing Test for Malformed JSON in ValidatedJson Extractor
**Issue:** The `ValidatedJson` extractor correctly returns a 422 Unprocessable Entity when `serde_json` fails to parse a request body with malformed JSON, but this `Err` execution path had zero unit test coverage.
**Learning:** `TestClient.post` implicitly serializes typed structs into perfectly formed JSON. To test how extractors handle invalid syntax (like missing quotes or braces), the test must manually override the request body with a raw `ukiapi::body::Body::from("...")` containing broken JSON string literals.
**Prevention:** Always write dedicated unit tests for request extractors that send intentionally malformed payload structures (not just invalid logical values) to ensure the framework's default deserialization error mapping functions as expected and doesn't leak internal panics.
