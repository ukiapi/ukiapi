## 2024-05-18 - Hardcoded Fallback Secrets
**Vulnerability:** A fallback secret was hardcoded for `INTERNAL_SECRET` (`development_secret`) in `example/src/routes.rs`.
**Learning:** Defaulting to a hardcoded string when an environment variable is missing allows attackers to bypass security if they can predict or extract the fallback secret. It creates a false sense of security where an application appears to be using environment secrets but silently falls back to a known value.
**Prevention:** Always fail securely by throwing an error or panicking if a required secret is missing, rather than providing a default fallback. Use `map_err` to return an appropriate HTTP exception instead of `unwrap_or_else` with a hardcoded string.
