# Docker sandbox commands for leet-tui development with Claude Code

.PHONY: build sandbox shell test clean rebuild

# Build the sandbox image
build:
	docker-compose build

# Start sandbox with Claude Code (main command)
sandbox:
	docker-compose run --rm sandbox

# Alias for sandbox
shell: sandbox

# Start sandbox and run Claude Code directly
claude:
	docker-compose run --rm sandbox claude

# Run the multi-language test suite
test:
	docker-compose run --rm sandbox ./scripts/test_languages.sh

# Run Python test runner
test-python:
	docker-compose run --rm sandbox python3 scripts/test_runner.py

# Build the Rust application
cargo-build:
	docker-compose run --rm sandbox cargo build

# Build release version
cargo-release:
	docker-compose run --rm sandbox cargo build --release

# Run cargo tests
cargo-test:
	docker-compose run --rm sandbox cargo test

# Run the TUI app
run:
	docker-compose run --rm sandbox cargo run

# Run clippy lints
lint:
	docker-compose run --rm sandbox cargo clippy

# Format code
fmt:
	docker-compose run --rm sandbox cargo fmt

# Clean Docker volumes and rebuild
clean:
	docker-compose down -v
	docker-compose build --no-cache

# Rebuild without cache
rebuild:
	docker-compose build --no-cache

# Stop all containers
stop:
	docker-compose down
