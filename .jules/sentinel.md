## 2024-05-18 - Path Traversal in UploadFile
**Vulnerability:** The filename extraction logic in the `UploadFile` extractor (`s.rsplit('/').next()`) did not properly sanitize paths containing `..` or `.`, allowing path traversal when saving uploaded files.
**Learning:** Simple string splitting is insufficient for path sanitization. Native path parsing functions provided by the standard library should be used to securely extract file names.
**Prevention:** Use `std::path::Path::new(filename).file_name()` to extract the file name, as it correctly handles `..` and `.` components, returning `None` in those cases.
