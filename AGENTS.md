# Agent instructions for RustAPI

These instructions guide agents in contributing to Rust projects, ensuring adherence to best practices, maintainability, and code quality.

## 1. General Principles

*   **DRY (Don't Repeat Yourself):**
    *   Identify and abstract common patterns into functions, traits, or macros.
    *   Leverage Rust's type system and generics to create reusable components.
    *   Avoid duplicating code blocks; refactor instead.
*   **SOLID Principles:**
    *   **Single Responsibility Principle (SRP):** Each module, struct, or function should have one clear responsibility.
    *   **Open/Closed Principle (OCP):** Software entities should be open for extension but closed for modification. Use traits and polymorphism.
    *   **Liskov Substitution Principle (LSP):** Subtypes must be substitutable for their base types. Relevant for trait objects.
    *   **Interface Segregation Principle (ISP):** Clients should not be forced to depend on interfaces they do not use. Prefer smaller, specific traits over large, general ones.
    *   **Dependency Inversion Principle (DIP):** Depend on abstractions, not concretions. Use traits for dependencies.

## 2. Code Style and Formatting

*   **Rustfmt:** Always run `cargo fmt` to ensure consistent code formatting.
*   **Clippy:** Adhere to `cargo clippy` suggestions to catch common mistakes and improve idiomatic Rust usage.
*   **Idiomatic Rust:** Write code that leverages Rust's ownership system, borrowing, pattern matching, and error handling (Result/Option) effectively.
*   **Naming Conventions:**
    *   `snake_case` for functions, variables, modules.
    *   `PascalCase` for types (structs, enums, traits).
    *   `SCREAMING_SNAKE_CASE` for constants.
*   **File Line Limit:** All Rust source files (`*.rs`) must not exceed 300 lines. This is enforced by the `make lint` command.

## 3. Dependency Management

*   **Cargo.toml:** Manage dependencies exclusively through `Cargo.toml`.
*   **Minimize Dependencies:** Only add necessary crates. Evaluate the impact of new dependencies on build times and binary size.
*   **Feature Flags:** Utilize Cargo's feature flags to enable/disable parts of a crate based on build configuration.
*   **License Compatibility:** Verify licenses of external crates for compatibility with the project's licensing.

## 4. Testing

*   **Unit Tests:** Write unit tests for individual functions and methods, typically within the same file using `#[cfg(test)] mod tests { ... }`.
*   **Integration Tests:** Create integration tests in the `tests/` directory to test the interaction between multiple modules or the entire crate as a black box.
*   **Documentation Tests:** Include code examples in `///` documentation comments that also serve as executable tests.
*   **Run Tests:** Always run `cargo test` after making changes to ensure nothing is broken.

## 5. Error Handling

*   **Result and Option:** Use `Result<T, E>` for recoverable errors and `Option<T>` for cases where a value might be absent.
*   **Avoid `unwrap()`/`expect()` in Library Code:** In general library code, avoid panicking with `unwrap()` or `expect()`. Propagate errors using `?` or handle them gracefully. Use `unwrap()` and `expect()` primarily in application entry points or tests where panicking is acceptable.
*   **Custom Error Types:** Define custom error enums for clarity and easier error handling. Consider using crates like `thiserror` or `anyhow`.

## 6. Documentation

*   **Public API:** Document all public structs, enums, functions, and traits using `///` comments. Explain their purpose, arguments, return values, and examples.
*   **Internal Details:** Use `//` comments for internal explanations of complex logic.
*   **`cargo doc`:** Ensure documentation builds correctly with `cargo doc`.

## 7. Security

*   **Input Validation:** Always validate external input rigorously to prevent vulnerabilities like injection attacks or buffer overflows.
*   **Dependency Auditing:** Regularly audit dependencies for known security vulnerabilities using tools like `cargo audit`.
*   **`unsafe` Code:** Minimize the use of `unsafe` blocks. When used, clearly document *why* it's necessary and ensure correctness and safety.

## 8. Performance

*   **Benchmarking:** If performance is critical, use `cargo bench` (with a benchmarking harness like `Criterion`) to measure and optimize code.
*   **Profiling:** Utilize profiling tools (e.g., `perf`, `Valgrind`) to identify performance bottlenecks.
*   **Efficient Algorithms:** Choose data structures and algorithms appropriate for the problem's constraints.

## 9. Pre-commit Hooks

*   **Validation on Commit:** The project uses `pre-commit` hooks to run `make lint` before commits. Ensure all staged Rust files adhere to the linting rules. If a pre-commit hook fails, resolve the issues before committing.
