## 2024-07-19 - Optimize Header Parsing in Rust Hot Paths
**Learning:** String allocation (`.to_lowercase()`) during request authentication (e.g., verifying Bearer tokens) creates significant overhead on every request.
**Action:** Use `.len()` checks and `.as_bytes()[..N].eq_ignore_ascii_case(b"...")` instead to avoid heap allocation while ensuring safe multi-byte boundaries.
