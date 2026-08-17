## 2024-07-22 - Missing Test for Error Obfuscation
**Issue:** `HTTPException` logic for hiding internal server error details (HTTP 500) behind generic messages lacked unit test coverage.
**Learning:** Security-critical features (like error detail obfuscation) in web framework responses must have explicit unit tests to prevent silent regressions during refactoring.
**Prevention:** Always add specific test cases for edge behaviors involving response transformation, ensuring sensitive context is not leaked to the client.
## 2024-07-22 - Missing Test for Multipart Form Fields
**Issue:** `UploadFile` extractor only parsed the very first multipart field and failed if metadata or non-file fields were positioned earlier in the HTTP request payload, which was untested.
**Learning:** `axum::extract::Multipart` iterates over fields sequentially. Extractors wrapping `Multipart` must iterate through all fields (e.g., using `while let Some(field)`) rather than assuming the file is always the first part.
**Prevention:** Always write unit tests for multipart body extraction that simulate varied field ordering (e.g., text fields before file fields) by explicitly constructing realistic boundary-delimited payloads.
## 2024-08-17 - Missing Test for StreamingResponse Error Path
**Issue:** The `Err` variant of `StreamingResponse`'s inner stream was untested when mapped to `axum::body::Body`.
**Learning:** In axum, stream errors generated from `Body::from_stream` are deferred until the stream is actively polled/collected via `to_bytes`, rather than failing eagerly during the `into_response()` conversion.
**Prevention:** Always assert on the `Result` from `axum::body::to_bytes` to verify deferred streaming errors.
