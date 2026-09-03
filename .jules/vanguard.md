## 2024-07-22 - Missing Test for Error Obfuscation
**Issue:** `HTTPException` logic for hiding internal server error details (HTTP 500) behind generic messages lacked unit test coverage.
**Learning:** Security-critical features (like error detail obfuscation) in web framework responses must have explicit unit tests to prevent silent regressions during refactoring.
**Prevention:** Always add specific test cases for edge behaviors involving response transformation, ensuring sensitive context is not leaked to the client.
## 2024-07-22 - Missing Test for Multipart Form Fields
**Issue:** `UploadFile` extractor only parsed the very first multipart field and failed if metadata or non-file fields were positioned earlier in the HTTP request payload, which was untested.
**Learning:** `axum::extract::Multipart` iterates over fields sequentially. Extractors wrapping `Multipart` must iterate through all fields (e.g., using `while let Some(field)`) rather than assuming the file is always the first part.
**Prevention:** Always write unit tests for multipart body extraction that simulate varied field ordering (e.g., text fields before file fields) by explicitly constructing realistic boundary-delimited payloads.
## 2024-07-22 - Missing Test for Extractor Serde Failures
**Issue:** Missing explicit tests to differentiate serde structural deserialization failures (e.g. passing a string when an object is expected) from custom validation constraint failures.
**Learning:** Extractors combining serde deserialization and custom validation (like `ValidatedJson` or `Query` in Ukiapi) have distinct failure domains. Serde deserialization failures must be independently tested by passing architecturally malformed payloads.
**Prevention:** Always test both serde structural failures (passing incompatible types/structures) and logical validation constraint failures (out of bounds/length constraints) on validation extractors.
