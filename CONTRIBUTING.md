# Contributing to UkiApi

Thank you for your interest in contributing to UkiApi! This document provides guidelines and information for contributors.

## Getting Started

1. Fork the repository on GitHub
2. Clone your fork locally:
   ```bash
   git clone git@github.com:your-username/ukiapi.git
   cd ukiapi
   ```
3. Create a new branch for your changes:
   ```bash
   git checkout -b feature/your-feature-name
   ```

## Development Setup

### Prerequisites

- Rust (latest stable version)
- Cargo

### Building

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

### Running Linting

```bash
make lint
```

This will:
- Run `cargo clippy` for Rust lints
- Check that all `.rs` files are under 300 lines

### Formatting

Always format your code before committing:

```bash
cargo fmt
```

## Code Style

- Follow Rust naming conventions (`snake_case` for functions/variables, `PascalCase` for types)
- Keep all Rust source files under 300 lines
- Write meaningful commit messages
- Add comments for complex logic
- Document public APIs with `///` doc comments

## Commit Guidelines

- Use clear, descriptive commit messages
- Start with a verb in imperative mood (e.g., "Add feature", "Fix bug", "Update docs")
- Reference issue numbers when applicable (e.g., "Fix #123")

## Pull Request Process

1. Ensure all tests pass (`cargo test`)
2. Ensure linting passes (`make lint`)
3. Update documentation if needed
4. Fill out the PR description with:
   - What changes were made
   - Why the changes were needed
   - Any breaking changes

## Reporting Issues

- Use the GitHub issue tracker
- Include a clear title and description
- Provide steps to reproduce the issue
- Include Rust version and OS information

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
