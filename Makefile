.PHONY: build test lint fmt clean install check

build:
	cargo build

release:
	cargo build --release

test:
	cargo test --workspace

lint:
	cargo clippy --workspace --all-targets -- -D warnings

fmt:
	cargo fmt

fmt-check:
	cargo fmt --check

check: fmt-check lint test

clean:
	cargo clean

install:
	cargo install --path crates/cli

run:
	cargo run -- $(ARGS)
