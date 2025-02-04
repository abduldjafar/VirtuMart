# Variables
RUSTFMT = rustfmt
CARGO = cargo
TARGET = $(shell basename $(shell pwd))

# Targets
.PHONY: all test full dev prod launch deployment show_usage run_tests run_dev run_prod

all: show_usage

# Display usage information
show_usage:
	@echo "Usage: make [test/full] [dev/prod] [launch/deployment]"
	@echo "Options:"
	@echo "  full       - Run formatting, Clippy tests, and the project."
	@echo "  test       - Run formatting and Clippy tests only."
	@echo "  dev        - Run the project in development mode (cargo run)."
	@echo "  prod       - Build and run the binary in production mode."
	@echo "  launch     - Run all checks and execute the project."
	@echo "  deployment - Run checks only, skip Rust code execution."

# Run Rustfmt and Clippy tests
test: run_tests

# Run all tests and then execute based on mode
full: run_tests
	@if [ "$(MODE)" = "dev" ]; then \
		$(MAKE) run_dev; \
	elif [ "$(MODE)" = "prod" ]; then \
		$(MAKE) run_prod; \
	else \
		$(MAKE) show_usage; \
		exit 1; \
	fi

# Run checks and launch in development mode
launch: run_tests run_dev

# Run checks but do not execute the Rust binary
deployment: run_tests

# Format code and check for warnings
run_tests:
	@echo "Running rustfmt..."
	@$(CARGO) fmt --all
	@echo "Checking file formatting..."
	@if ! $(CARGO) fmt --all -- --check; then \
		echo "Formatting check failed. Exiting..."; \
		exit 1; \
	fi
	@echo "Formatting check passed!"
	@echo "Running Clippy..."
	@if ! $(CARGO) clippy -- -D warnings; then \
		echo "Clippy check failed. Exiting..."; \
		exit 1; \
	fi
	@echo "Clippy check passed!"

# Run the project in development mode
run_dev:
	@echo "Running in development mode..."
	@$(CARGO) run

# Build and run the project in production mode
run_prod:
	@echo "Building for production..."
	@$(CARGO) build --release
	@echo "Running in production mode..."
	@./target/release/$(TARGET)

# Push code to GitHub with a dynamic commit message
push_code: run_tests
	@echo "Pushing code to GitHub..."
	@if [ -z "$(COMMIT_MSG)" ]; then \
        echo "Commit message is required. Use COMMIT_MSG='your commit message'"; \
        exit 1; \
	fi
	@if git diff-index --quiet HEAD --; then \
		echo "Nothing to commit, working tree clean"; \
    else \
		git add .; \
		git commit -m "$(COMMIT_MSG)"; \
		git push; \
	fi