## 2024-07-16 - Avoid unnecessary clones when destructing serde_json::Value
**Learning:** When processing a `serde_json::Value`, specifically an Object map, `remove()` returns the owned value. By matching `Value::Object` when removing a key like "definitions", we can iterate and transfer ownership of its keys and values to another map directly, avoiding `k.clone()` and `v.clone()`.
**Action:** Always look for opportunities to transfer ownership directly using destructuring and `remove()` when restructuring JSON data in Rust instead of falling back to `.clone()`.
