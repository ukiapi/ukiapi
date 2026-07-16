## 2025-01-20 - Hardcoded Internal Secret Fallback

**Vulnerability:** A hardcoded string `"development_secret"` was being used as a fallback if the `INTERNAL_SECRET` environment variable wasn't set.
**Learning:** Providing hardcoded defaults for sensitive variables masks deployment misconfigurations and leaks into the resulting system artifacts (like the mock `ItemDb`). It is safer to fail loudly than to proceed with insecure defaults.
**Prevention:** Avoid `unwrap_or_else` or `unwrap_or` for environment variables meant to be secret. Always propagate the error and return an appropriate HTTP exception (e.g., `500 Internal Server Error`).
