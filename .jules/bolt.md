## 2024-07-20 - Fast prefix checking in string processing
**Learning:** In Rust, checking prefixes in a hot path (like HTTP headers or URLs) by converting the entire string using `.to_lowercase()` causes unnecessary heap allocations.
**Action:** Use `.as_bytes()` combined with `.eq_ignore_ascii_case(b"...")` to avoid heap allocations and improve performance in text processing hot paths.
