.DEFAULT_GOAL := help

CARGO ?= cargo

OLD_KEBAB := rust-template
OLD_SNAKE := rust_template

.PHONY: help build release run test fmt fmt-check clippy check doc clean install-hooks rename

help: ## Show this help
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-14s\033[0m %s\n", $$1, $$2}'

build: ## Build debug binary
	$(CARGO) build --all-features

release: ## Build optimized binary
	$(CARGO) build --release --all-features

run: ## Run the binary
	$(CARGO) run

test: ## Run tests
	$(CARGO) test --all-features

fmt: ## Format code
	$(CARGO) fmt --all

fmt-check: ## Check formatting without modifying files
	$(CARGO) fmt --all -- --check

clippy: ## Lint with clippy (warnings as errors)
	$(CARGO) clippy --all-targets --all-features -- -D warnings

check: fmt-check clippy test ## Run fmt-check + clippy + test

doc: ## Build and open docs
	$(CARGO) doc --no-deps --open

clean: ## Remove build artifacts
	$(CARGO) clean

install-hooks: ## Install the pre-commit git hooks
	pre-commit install

rename: ## Rename the template crate: make rename NAME=my-cool-service
	@if [ -z "$(NAME)" ]; then \
		echo "Usage: make rename NAME=my-cool-service" >&2; \
		exit 1; \
	fi
	@echo "$(NAME)" | grep -Eq '^[a-z][a-z0-9-]*$$' || { \
		echo "Error: NAME must be lowercase kebab-case (e.g. my-cool-service)" >&2; \
		exit 1; \
	}
	@NEW_SNAKE=$$(echo "$(NAME)" | tr '-' '_'); \
	for f in Cargo.toml README.md src/main.rs tests/integration_test.rs; do \
		[ -f "$$f" ] && perl -pi -e "s/\Q$(OLD_KEBAB)\E/$(NAME)/g; s/\Q$(OLD_SNAKE)\E/$$NEW_SNAKE/g" "$$f"; \
	done; \
	echo "Renamed project to '$(NAME)' (crate: $$NEW_SNAKE)."
