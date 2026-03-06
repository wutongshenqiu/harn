# harn

Universal project harness with SDD methodology. Rust workspace CLI tool.

## Architecture

```
crates/
  cli/        # Binary: clap CLI + dialoguer interactive prompts
  core/       # Library: config types, Module trait, ProjectContext
  modules/    # Library: 9 built-in modules (sdd, ci, agent, build, ...)
  templates/  # Library: minijinja engine + include_dir compile-time embedding

templates/    # Template files embedded into the binary at compile time
```

## Key Patterns

- **Module trait** (`crates/core/src/module.rs`): All modules implement `id()`, `name()`, `description()`, `generate(ctx)`
- **Registry** (`crates/modules/src/registry.rs`): Modules registered in execution order
- **Templates** embedded via `include_dir!` — zero runtime file dependencies
- **Config** (`harn.toml`): TOML-based, defines project + stacks + per-module config

## Commands

```bash
make check        # fmt-check + clippy + test (full CI locally)
make lint         # cargo clippy
make test         # cargo test --workspace
make fmt          # cargo fmt
make build        # cargo build
make release      # cargo build --release
```

## Extension Points

- New module: implement Module trait, register in registry.rs, add templates
- New CI provider: add templates in `templates/ci/<provider>/`, update `crates/modules/src/ci.rs`
- New language: add Makefile template, .gitignore fragment, linter config, CI steps
- New AI tool: add template in `templates/agent/`, update `crates/modules/src/agent.rs`

## Conventions

- Edition 2024, rust-version 1.85, resolver 3
- All deps in workspace Cargo.toml
- `cargo clippy -- -D warnings` must pass
- Templates use minijinja syntax: `{{ variable }}`
