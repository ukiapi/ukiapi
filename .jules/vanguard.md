## 2024-07-22 - Missing Test for Error Obfuscation
**Issue:** `HTTPException` logic for hiding internal server error details (HTTP 500) behind generic messages lacked unit test coverage.
**Learning:** Security-critical features (like error detail obfuscation) in web framework responses must have explicit unit tests to prevent silent regressions during refactoring.
**Prevention:** Always add specific test cases for edge behaviors involving response transformation, ensuring sensitive context is not leaked to the client.
## 2024-07-22 - Missing Test for Multipart Form Fields
**Issue:** `UploadFile` extractor only parsed the very first multipart field and failed if metadata or non-file fields were positioned earlier in the HTTP request payload, which was untested.
**Learning:** `axum::extract::Multipart` iterates over fields sequentially. Extractors wrapping `Multipart` must iterate through all fields (e.g., using `while let Some(field)`) rather than assuming the file is always the first part.
**Prevention:** Always write unit tests for multipart body extraction that simulate varied field ordering (e.g., text fields before file fields) by explicitly constructing realistic boundary-delimited payloads.
## 2024-07-22 - Global State Mutations in Tests
**Issue:** Tests modifying environment variables (`std::env::set_var`) concurrently fail intermittently or cause deadlocks/panics when using global locks incorrectly.
**Learning:** Mutating global environment variables inside multithreaded test runners (like `cargo test`) requires synchronization. Standard `std::sync::Mutex` guards cannot be held across `.await` points without triggering clippy errors and potential deadlocks in Tokio.
**Prevention:** Use an async-aware lock (like `tokio::sync::Mutex`) when serializing test access to global state across `.await` points, or structure the code to allow dependency injection of configuration rather than relying on global environment variables.
