## 2024-07-22 - Missing Test for Error Obfuscation
**Issue:** `HTTPException` logic for hiding internal server error details (HTTP 500) behind generic messages lacked unit test coverage.
**Learning:** Security-critical features (like error detail obfuscation) in web framework responses must have explicit unit tests to prevent silent regressions during refactoring.
**Prevention:** Always add specific test cases for edge behaviors involving response transformation, ensuring sensitive context is not leaked to the client.
## 2024-07-22 - Missing Test for Multipart Form Fields
**Issue:** `UploadFile` extractor only parsed the very first multipart field and failed if metadata or non-file fields were positioned earlier in the HTTP request payload, which was untested.
**Learning:** `axum::extract::Multipart` iterates over fields sequentially. Extractors wrapping `Multipart` must iterate through all fields (e.g., using `while let Some(field)`) rather than assuming the file is always the first part.
**Prevention:** Always write unit tests for multipart body extraction that simulate varied field ordering (e.g., text fields before file fields) by explicitly constructing realistic boundary-delimited payloads.
## 2024-08-13 - Missing Test for Extractor Serde Failure Path
**Issue:** `ValidatedJson` and `Query` extractors lacked test coverage for the serde deserialization failure path (e.g., malformed JSON syntax or type mismatches). The existing tests only validated business logic constraints via the `validator` crate (like `.validate()`).
**Learning:** Extractors that perform both structural decoding (serde) and semantic validation (`validator`) have two distinct points of failure. Testing only semantic validation leaves the structural error responses untested.
**Prevention:** Always write separate unit tests that send structurally invalid payloads (e.g., strings instead of expected structs, or raw text instead of JSON) to verify the correct HTTP 422 error format is returned by the early serde deserialization abort path.
