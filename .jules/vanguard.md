## 2024-07-22 - Missing Test for Error Obfuscation
**Issue:** `HTTPException` logic for hiding internal server error details (HTTP 500) behind generic messages lacked unit test coverage.
**Learning:** Security-critical features (like error detail obfuscation) in web framework responses must have explicit unit tests to prevent silent regressions during refactoring.
**Prevention:** Always add specific test cases for edge behaviors involving response transformation, ensuring sensitive context is not leaked to the client.
## 2024-07-22 - Missing Test for Multipart Form Fields
**Issue:** `UploadFile` extractor only parsed the very first multipart field and failed if metadata or non-file fields were positioned earlier in the HTTP request payload, which was untested.
**Learning:** `axum::extract::Multipart` iterates over fields sequentially. Extractors wrapping `Multipart` must iterate through all fields (e.g., using `while let Some(field)`) rather than assuming the file is always the first part.
**Prevention:** Always write unit tests for multipart body extraction that simulate varied field ordering (e.g., text fields before file fields) by explicitly constructing realistic boundary-delimited payloads.
## 2024-07-22 - Missing Test for Empty JSON Body Extractor
**Issue:** `ValidatedJson` extractor did not have explicit coverage for empty payload scenarios. It successfully rejected invalid schemas (e.g. invalid json format, empty string) returning 422 Unprocessable Entity, but this behavior wasn't documented explicitly through a test case.
**Learning:** Framework capabilities regarding default extractor error handling (e.g., rejecting an empty string `&""` payload for a typed JSON struct) need explicit unit testing to document expected failure paths.
**Prevention:** Always write unit tests checking edge case inputs such as missing, zero-length, and malformed bodies for custom extractors.
