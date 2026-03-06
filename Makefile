.PHONY: build release test lint fmt fmt-check check clean install run help

build:  ## Build debug
	cargo build

release:  ## Build release
	cargo build --release

test:  ## Run tests
	cargo test --workspace

lint:  ## Lint (clippy + fmt check)
	cargo fmt --check
	cargo clippy --workspace --all-targets -- -D warnings

fmt:  ## Format code
	cargo fmt

check:  ## Full CI locally (fmt + lint + test)
	cargo fmt --check
	cargo clippy --workspace --all-targets -- -D warnings
	cargo test --workspace

clean:  ## Clean artifacts
	cargo clean

install:  ## Install to cargo bin
	cargo install --path crates/cli

run:  ## Run with args (make run ARGS="init .")
	cargo run -- $(ARGS)

help:  ## Show help
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

.DEFAULT_GOAL := help
