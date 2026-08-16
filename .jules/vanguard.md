## 2024-07-22 - Missing Test for Error Obfuscation
**Issue:** `HTTPException` logic for hiding internal server error details (HTTP 500) behind generic messages lacked unit test coverage.
**Learning:** Security-critical features (like error detail obfuscation) in web framework responses must have explicit unit tests to prevent silent regressions during refactoring.
**Prevention:** Always add specific test cases for edge behaviors involving response transformation, ensuring sensitive context is not leaked to the client.
## 2024-07-22 - Missing Test for Multipart Form Fields
**Issue:** `UploadFile` extractor only parsed the very first multipart field and failed if metadata or non-file fields were positioned earlier in the HTTP request payload, which was untested.
**Learning:** `axum::extract::Multipart` iterates over fields sequentially. Extractors wrapping `Multipart` must iterate through all fields (e.g., using `while let Some(field)`) rather than assuming the file is always the first part.
**Prevention:** Always write unit tests for multipart body extraction that simulate varied field ordering (e.g., text fields before file fields) by explicitly constructing realistic boundary-delimited payloads.
## 2024-08-16 - Missing Tests for Extractor Deserialization Failures
**Issue:** `ValidatedJson` and `Query` extractors only had tests for successful cases and validation failures, missing the explicit failure domain of `serde` deserialization failures.
**Learning:** When testing extractors that combine serde deserialization and custom validation (like `ValidatedJson` or `Query` in Ukiapi), explicitly test both failure domains: serde deserialization failures (e.g., sending structural mismatches like a string instead of an object) and validation constraint failures (e.g., out-of-range values).
**Prevention:** Always include test cases for structurally invalid JSON and invalid query parameter types when writing tests for custom extractors.
