## 2024-07-22 - Missing Test for Error Obfuscation
**Issue:** `HTTPException` logic for hiding internal server error details (HTTP 500) behind generic messages lacked unit test coverage.
**Learning:** Security-critical features (like error detail obfuscation) in web framework responses must have explicit unit tests to prevent silent regressions during refactoring.
**Prevention:** Always add specific test cases for edge behaviors involving response transformation, ensuring sensitive context is not leaked to the client.
## 2024-07-22 - Missing Test for Multipart Form Fields
**Issue:** `UploadFile` extractor only parsed the very first multipart field and failed if metadata or non-file fields were positioned earlier in the HTTP request payload, which was untested.
**Learning:** `axum::extract::Multipart` iterates over fields sequentially. Extractors wrapping `Multipart` must iterate through all fields (e.g., using `while let Some(field)`) rather than assuming the file is always the first part.
**Prevention:** Always write unit tests for multipart body extraction that simulate varied field ordering (e.g., text fields before file fields) by explicitly constructing realistic boundary-delimited payloads.
## 2024-08-22 - Flaky Tests Due to Shared Environment Variables
**Issue:** `test_auth_flow` and `test_auth_invalid_token` in `example/tests/test_auth.rs` were flaky because they concurrently mutated the global `JWT_SECRET` environment variable using `std::env::set_var`.
**Learning:** In async Rust tests (`#[tokio::test]`), mutating global state like environment variables causes race conditions across multithreaded test runners.
**Prevention:** Isolate environment variable mutations in tests using the `serial_test` crate (`#[serial_test::serial]`) to prevent cross-test pollution and guarantee sequential execution.
