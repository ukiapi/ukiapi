## 2024-07-14 - Optimize OpenAPI Schema Processing
**Learning:** Top-level cloning of OpenAPI schemas during router build/schema processing incurs unnecessary overhead. The values can instead be consumed by removing them from owned parent structs (via `take()`).
**Action:** Always consume complex JSON objects and nested structs directly if they are no longer needed, bypassing `value.clone()` and deep traversal clones.
