## 2024-07-22 - Missing Test for Error Obfuscation
**Issue:** `HTTPException` logic for hiding internal server error details (HTTP 500) behind generic messages lacked unit test coverage.
**Learning:** Security-critical features (like error detail obfuscation) in web framework responses must have explicit unit tests to prevent silent regressions during refactoring.
**Prevention:** Always add specific test cases for edge behaviors involving response transformation, ensuring sensitive context is not leaked to the client.
## 2024-07-22 - Missing Test for Multipart Form Fields
**Issue:** `UploadFile` extractor only parsed the very first multipart field and failed if metadata or non-file fields were positioned earlier in the HTTP request payload, which was untested.
**Learning:** `axum::extract::Multipart` iterates over fields sequentially. Extractors wrapping `Multipart` must iterate through all fields (e.g., using `while let Some(field)`) rather than assuming the file is always the first part.
**Prevention:** Always write unit tests for multipart body extraction that simulate varied field ordering (e.g., text fields before file fields) by explicitly constructing realistic boundary-delimited payloads.
## 2024-07-30 - Missing Test for ScopedDepends Error Extraction
**Issue:** `ScopedDepends` logic correctly wrapped errors returned by `ScopedDependency::resolve` into `HTTPException` responses, but this boundary behavior lacked unit test coverage.
**Learning:** In Rust, testing `Result::unwrap_err()` in unit tests requires both the `Ok` and `Err` types to implement the `std::fmt::Debug` trait. Missing `#[derive(Debug)]` on internal extractor wrappers (like `ScopedDepends`) or mock test structs can cause compilation failures when trying to assert against the error variants.
**Prevention:** Always ensure internal extractor wrapper structs and dummy structs created exclusively for testing implement `Debug` (via `#[derive(Debug)]`) when they might be used in `unwrap()`, `unwrap_err()`, or `assert_eq!` calls.
