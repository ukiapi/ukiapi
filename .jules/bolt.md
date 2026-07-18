## 2024-03-24 - String allocation in Rust hot paths
**Learning:** Checking prefixes case-insensitively with `.to_lowercase().starts_with(...)` causes unnecessary heap allocations in Rust.
**Action:** When validating headers or other strings in hot paths (like HTTP middlewares/extractors), always use a length check and `.eq_ignore_ascii_case()` on a string slice (e.g., `s.len() >= 7 && s[..7].eq_ignore_ascii_case("prefix ")`).
