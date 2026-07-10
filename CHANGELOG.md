# Changelog

## v0.0.1

### Changed
- **BREAKING:** Rename project from `ukidama` to `ukiapi`
- Update all package names: `ukiapi`, `ukiapi-macros`
- Update environment variables: `UKIAPI_HOST`, `UKIAPI_PORT`
- Update CLI banners and help text to reference UkiApi

### Added
- MIT License file
- CI badge and license badge in README
- Contributing guidelines (CONTRIBUTING.md)
- Health check endpoint for K8s readiness/liveness probes
- Scoped Dependency Injection support
- Dynamic response projection with field inclusion/exclusion

### Fixed
- Path traversal vulnerability in `UploadFile` extraction
- Hardcoded JWT secret fallback

### Removed
- Delete `STATE_OF_THE_ART.md`
- Remove AI restriction policy from AGENTS.md

## v0.1.5

### Added
- Implement compile-time zero-boilerplate route autodiscovery and merging
- Add integrated authentication and authorization helpers with JWT-based custom claim extraction
- Expose `HTTPConnection` details for granular connection access (client IP, headers, method, URI)
- Full integration of WebSockets support with `#[websocket]` macro and `WebSocketUpgrade` extractor

### Fixed
- Fix path traversal vulnerability in `UploadFile` extraction
- Downgrade `simple_asn1` and `time` dependencies for `rustc 1.86.0` compatibility

## v0.1.2

### Fixed
- Export missing `responses` and `utils` modules from ukiapi crate
- Fix example imports for `HTMLResponse`, `RedirectResponse`, `FileResponse`, `BackgroundTasks`, `UploadFile`, and `jsonable_encoder`

## v0.1.1

### Added
- Integrate WebSockets support with `#[websocket]` macro and `WebSocketUpgrade` extractor

### Fixed
- Auto-detect binary name in dev/run commands instead of hardcoding `example`
- `uki new` template uses git dependencies instead of broken path deps

## v0.1.0

### Added
- Initialize UkiApi project with axum-based routing, validation, and automated documentation generation
- Add `new` command and `ukiapi-new` binary for project scaffolding
- Implement APIRouter and support for route tags
- Add lifecycle handlers, static files, and unified Request
- Implement custom response classes and jsonable_encoder
- Consolidate separate examples into the main example project
- Pre-commit hooks for linting and file line limits

### Fixed
- Resolve clippy warnings
- Correct boilerplate imports in generated projects
- Use path dependencies for ukiapi and ukiapi-macros in new projects

### Refactored
- Update CLI entry point to align with internal module restructuring
