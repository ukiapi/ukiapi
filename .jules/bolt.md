## 2024-11-20 - Lazy Iterators for Memory Optimization
**Learning:** Found a common anti-pattern where a Rust iterator was performing `.map()` (which included expensive string allocations via `.clone()`) and `.collect()` *before* truncating the vector to a limit. Because iterators in Rust are lazy, calling `.take(limit)` *before* `.map()` defers allocations, restricting expensive cloning only to the items that actually need it.
**Action:** Always verify if `.truncate()` or slice limits on vectors can be replaced with `.take()` placed strategically earlier in an iterator chain to avoid unnecessary processing and memory allocation.
## 2024-07-20 - Fast prefix checking in string processing
**Learning:** In Rust, checking prefixes in a hot path (like HTTP headers or URLs) by converting the entire string using `.to_lowercase()` causes unnecessary heap allocations.
**Action:** Use `.as_bytes()` combined with `.eq_ignore_ascii_case(b"...")` to avoid heap allocations and improve performance in text processing hot paths.
