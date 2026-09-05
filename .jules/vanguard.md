## 2024-07-22 - Missing Test for Error Obfuscation
**Issue:** `HTTPException` logic for hiding internal server error details (HTTP 500) behind generic messages lacked unit test coverage.
**Learning:** Security-critical features (like error detail obfuscation) in web framework responses must have explicit unit tests to prevent silent regressions during refactoring.
**Prevention:** Always add specific test cases for edge behaviors involving response transformation, ensuring sensitive context is not leaked to the client.
## 2024-07-22 - Missing Test for Multipart Form Fields
**Issue:** `UploadFile` extractor only parsed the very first multipart field and failed if metadata or non-file fields were positioned earlier in the HTTP request payload, which was untested.
**Learning:** `axum::extract::Multipart` iterates over fields sequentially. Extractors wrapping `Multipart` must iterate through all fields (e.g., using `while let Some(field)`) rather than assuming the file is always the first part.
**Prevention:** Always write unit tests for multipart body extraction that simulate varied field ordering (e.g., text fields before file fields) by explicitly constructing realistic boundary-delimited payloads.
## 2024-07-22 - Missing Test for Extractor Deserialization Failure
**Issue:** Extractor failure due to serde deserialization (e.g., missing parameter, wrong type) lacked unit test coverage compared to validation constraints.
**Learning:** Testing only custom validation is not enough; the outer deserialization layer must also be tested. In `TestClient`, a generic string like `&"invalid"` can trigger JSON struct deserialization failure.
**Prevention:** Ensure extractors explicitly test failure modes at both the serde JSON structural boundary (deserialization) and the subsequent internal business validation boundary.
