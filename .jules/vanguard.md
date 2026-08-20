## 2024-07-22 - Missing Test for Error Obfuscation
**Issue:** `HTTPException` logic for hiding internal server error details (HTTP 500) behind generic messages lacked unit test coverage.
**Learning:** Security-critical features (like error detail obfuscation) in web framework responses must have explicit unit tests to prevent silent regressions during refactoring.
**Prevention:** Always add specific test cases for edge behaviors involving response transformation, ensuring sensitive context is not leaked to the client.
## 2024-07-22 - Missing Test for Multipart Form Fields
**Issue:** `UploadFile` extractor only parsed the very first multipart field and failed if metadata or non-file fields were positioned earlier in the HTTP request payload, which was untested.
**Learning:** `axum::extract::Multipart` iterates over fields sequentially. Extractors wrapping `Multipart` must iterate through all fields (e.g., using `while let Some(field)`) rather than assuming the file is always the first part.
**Prevention:** Always write unit tests for multipart body extraction that simulate varied field ordering (e.g., text fields before file fields) by explicitly constructing realistic boundary-delimited payloads.
## 2024-07-23 - Flaky async tests modifying environment variables
**Issue:** Tests modifying environment variables using `std::env::set_var` in `#[tokio::test]` were prone to race conditions during parallel execution.
**Learning:** In Rust asynchronous tests, mutating environment variables via `std::env::set_var` causes race conditions because of the shared global state across multithreaded runners.
**Prevention:** Always use the `serial_test` crate (e.g., `#[serial_test::serial]`) to safely isolate tests that modify global state like environment variables, ensuring deterministic test execution.
