# Changelog

## v0.1.2

### Fixed
- Export missing `responses` and `utils` modules from rustapi crate
- Fix example imports for `HTMLResponse`, `RedirectResponse`, `FileResponse`, `BackgroundTasks`, `UploadFile`, and `jsonable_encoder`

## v0.1.1
n### Added
- Integrate WebSockets support with `#[websocket]` macro and `WebSocketUpgrade` extractor

### Fixed
- Auto-detect binary name in dev/run commands instead of hardcoding `example`
- `rustapi new` template uses git dependencies instead of broken path deps

## v0.1.0

### Added
- Initialize RustAPI project with axum-based routing, validation, and automated documentation generation
- Add `new` command and `rustapi-new` binary for project scaffolding
- Implement APIRouter and support for route tags
- Add lifecycle handlers, static files, and unified Request
- Implement custom response classes and jsonable_encoder
- Consolidate separate examples into the main example project
- Pre-commit hooks for linting and file line limits

### Fixed
- Resolve clippy warnings
- Correct boilerplate imports in generated projects
- Use path dependencies for rustapi and rustapi-macros in new projects

### Refactored
- Update CLI entry point to align with internal module restructuring
