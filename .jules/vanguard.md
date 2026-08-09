## 2024-07-22 - Missing Test for Error Obfuscation
**Issue:** `HTTPException` logic for hiding internal server error details (HTTP 500) behind generic messages lacked unit test coverage.
**Learning:** Security-critical features (like error detail obfuscation) in web framework responses must have explicit unit tests to prevent silent regressions during refactoring.
**Prevention:** Always add specific test cases for edge behaviors involving response transformation, ensuring sensitive context is not leaked to the client.
## 2024-07-22 - Missing Test for Multipart Form Fields
**Issue:** `UploadFile` extractor only parsed the very first multipart field and failed if metadata or non-file fields were positioned earlier in the HTTP request payload, which was untested.
**Learning:** `axum::extract::Multipart` iterates over fields sequentially. Extractors wrapping `Multipart` must iterate through all fields (e.g., using `while let Some(field)`) rather than assuming the file is always the first part.
**Prevention:** Always write unit tests for multipart body extraction that simulate varied field ordering (e.g., text fields before file fields) by explicitly constructing realistic boundary-delimited payloads.
## 2024-08-09 - Missing Tests for Extractor Parsing Failures
**Issue:** The `Query` and `ValidatedJson` extractors only had tests verifying application-level validation failures (e.g., returning 422 on negative integers). They lacked test coverage for underlying parsing failures, such as structurally invalid JSON payloads or wrong query parameter data types.
**Learning:** Testing validation logic is not enough. You must also explicitly test the deserialization boundaries of extractors to ensure the framework handles and obfuscates parsing errors correctly according to the expected response format.
**Prevention:** When creating custom axum extractors, write tests that pass completely malformed strings (like `{ "broken": `) or invalid type permutations (like string for int) to ensure the 422 HTTP responses and error messages behave deterministically.
