## 2024-07-17 - Avoid to_lowercase() for prefix checking
**Learning:** Calling .to_lowercase().starts_with() on a potentially long string (like an Authorization header with a JWT) allocates a new String for the entire text, which causes a measurable performance drop in a hot path.
**Action:** Use string slicing combined with eq_ignore_ascii_case() (e.g., .get(..7).is_some_and(|s| s.eq_ignore_ascii_case("bearer "))) to check prefixes without allocation.
