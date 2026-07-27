## 2024-07-22 - Missing Test for Error Obfuscation
**Issue:** `HTTPException` logic for hiding internal server error details (HTTP 500) behind generic messages lacked unit test coverage.
**Learning:** Security-critical features (like error detail obfuscation) in web framework responses must have explicit unit tests to prevent silent regressions during refactoring.
**Prevention:** Always add specific test cases for edge behaviors involving response transformation, ensuring sensitive context is not leaked to the client.

## 2024-10-27 - Test for Multipart Upload Field Parsing
**Issue:** `UploadFile` extractor incorrectly assumed the target file was always the very first field in a multipart form request.
**Learning:** Multipart form requests can contain preceding fields (like metadata) before the actual file payload. Tests must explicitly simulate requests where the file is *not* the first boundary segment.
**Prevention:** Always write integration tests for multipart parsing that include leading text fields before the file to verify the loop/iteration logic correctly scans for the file name.
