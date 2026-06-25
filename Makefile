lint:
	@echo "Running Rust lints and line count checks..."
	@cargo clippy -- -D warnings
	@./scripts/check_line_counts.sh

.PHONY: lint