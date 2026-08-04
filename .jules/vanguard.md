## 2024-07-22 - Missing Test for Error Obfuscation
**Issue:** `HTTPException` logic for hiding internal server error details (HTTP 500) behind generic messages lacked unit test coverage.
**Learning:** Security-critical features (like error detail obfuscation) in web framework responses must have explicit unit tests to prevent silent regressions during refactoring.
**Prevention:** Always add specific test cases for edge behaviors involving response transformation, ensuring sensitive context is not leaked to the client.
## 2024-07-22 - Missing Test for Multipart Form Fields
**Issue:** `UploadFile` extractor only parsed the very first multipart field and failed if metadata or non-file fields were positioned earlier in the HTTP request payload, which was untested.
**Learning:** `axum::extract::Multipart` iterates over fields sequentially. Extractors wrapping `Multipart` must iterate through all fields (e.g., using `while let Some(field)`) rather than assuming the file is always the first part.
**Prevention:** Always write unit tests for multipart body extraction that simulate varied field ordering (e.g., text fields before file fields) by explicitly constructing realistic boundary-delimited payloads.
## 2024-05-24 - TestClient Enhancements
**Issue:** TestClient in `ukiapi` lacked support for PUT, DELETE, and PATCH HTTP methods, making it harder to test routes that depend on these operations.
**Learning:** Adding test-only methods requires ensuring that internal APIs use existing valid abstractions. The serialization logic (using `<T: Serialize>`) for bodies is directly translatable across mutation HTTP methods.
**Prevention:** When creating test abstraction utilities (like `TestClient`), ensure all standard HTTP verbs are exposed equivalently to avoid blocking other developers writing integration tests.
