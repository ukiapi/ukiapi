## 2024-07-22 - Missing Test for Error Obfuscation
**Issue:** `HTTPException` logic for hiding internal server error details (HTTP 500) behind generic messages lacked unit test coverage.
**Learning:** Security-critical features (like error detail obfuscation) in web framework responses must have explicit unit tests to prevent silent regressions during refactoring.
**Prevention:** Always add specific test cases for edge behaviors involving response transformation, ensuring sensitive context is not leaked to the client.
## 2024-07-22 - Missing Test for Multipart Form Fields
**Issue:** `UploadFile` extractor only parsed the very first multipart field and failed if metadata or non-file fields were positioned earlier in the HTTP request payload, which was untested.
**Learning:** `axum::extract::Multipart` iterates over fields sequentially. Extractors wrapping `Multipart` must iterate through all fields (e.g., using `while let Some(field)`) rather than assuming the file is always the first part.
**Prevention:** Always write unit tests for multipart body extraction that simulate varied field ordering (e.g., text fields before file fields) by explicitly constructing realistic boundary-delimited payloads.
## 2024-07-22 - Missing std::error::Error Implementation for ScopedDiError
**Issue:** `ScopedDiError` (a custom error type) lacked the `std::error::Error` trait implementation, preventing its interoperability with standard Rust error handling (like `Box<dyn Error>`).
**Learning:** When creating custom error types for public API boundaries (or core features), you must explicitly derive `Debug` and implement both `Display` and `std::error::Error` to adhere to standard Rust idiomatic error handling.
**Prevention:** Always add a unit test specifically asserting that a reference to the custom error can be coerced into `&dyn std::error::Error`, verifying standard library trait compatibility.
