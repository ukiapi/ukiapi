## 2024-06-26 - Hardcoded internal secret in examples

**Vulnerability:** A hardcoded `internal_secret` ("secret123") was present in the `create_item` endpoint in `example/src/routes.rs`.
**Learning:** Example projects often embed secrets directly to simplify setup for new users. However, these can inadvertently be deployed or copied directly into real projects by users assuming it's the standard practice.
**Prevention:** Use environment variables (e.g. `std::env::var("INTERNAL_SECRET")`) combined with `unwrap_or_else` or similar constructs to provide a seamless development fallback without embedding production secrets in the codebase itself. Ensure all scaffolded or example code follows production security practices.
