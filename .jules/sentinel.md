## 2025-01-20 - Prevent Path Traversal in Upload Handler
**Vulnerability:** Found a Path Traversal vulnerability in the `/upload` endpoint of `example/src/routes.rs`. The code blindly extracted `file.filename` and appended it to a temporary directory without checking for parent directory sequences (`../`).
**Learning:** `UploadFile` filenames sent by clients are untrusted input. Blindly joining a path can overwrite arbitrary files if the filename contains traversal characters (e.g. `../../../etc/passwd`).
**Prevention:** Sanitize the filename by using `std::path::Path::new(...).file_name()` to extract only the final file component, ignoring any directory paths.
