## 2024-07-22 - Missing Test for Error Obfuscation
**Issue:** `HTTPException` logic for hiding internal server error details (HTTP 500) behind generic messages lacked unit test coverage.
**Learning:** Security-critical features (like error detail obfuscation) in web framework responses must have explicit unit tests to prevent silent regressions during refactoring.
**Prevention:** Always add specific test cases for edge behaviors involving response transformation, ensuring sensitive context is not leaked to the client.
## 2024-07-22 - Missing Test for Multipart Form Fields
**Issue:** `UploadFile` extractor only parsed the very first multipart field and failed if metadata or non-file fields were positioned earlier in the HTTP request payload, which was untested.
**Learning:** `axum::extract::Multipart` iterates over fields sequentially. Extractors wrapping `Multipart` must iterate through all fields (e.g., using `while let Some(field)`) rather than assuming the file is always the first part.
**Prevention:** Always write unit tests for multipart body extraction that simulate varied field ordering (e.g., text fields before file fields) by explicitly constructing realistic boundary-delimited payloads.
## 2024-07-22 - Missing Test for Extractor Validation with Deserialization Errors
**Issue:** Extractor failures inside UkiAPI's TestClient were only tested against field-level constraint validations, missing structural JSON parsing errors (e.g. sending a generic string instead of a valid JSON object).
**Learning:** `ValidatedJson` and `Query` extractors parse incoming text twice: once structurally (serde deserialization) and once semantically (validation constraints).
**Prevention:** Explicitly pass mismatched structural primitive literals (like `&"invalid"`) alongside structurally correct but constraint-violating structs to ensure both the validation pipeline AND deserialization pipeline appropriately return 422 errors.
