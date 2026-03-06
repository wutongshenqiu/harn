# Contributing to harn

## Development Setup

```bash
git clone https://github.com/nicepkg/harn.git
cd harn
cargo build
```

## Project Structure

```
crates/
├── cli/        # Binary: CLI entry point (clap + dialoguer)
├── core/       # Library: config types, Module trait, context
├── modules/    # Library: 9 built-in modules (sdd, ci, agent, ...)
└── templates/  # Library: template engine (minijinja + include_dir)

templates/      # Template files (embedded at compile time)
```

## Adding a New Module

1. Create `crates/modules/src/your_module.rs`
2. Implement `Module` trait (id, name, description, generate)
3. Register in `crates/modules/src/registry.rs`
4. Add config struct in `crates/core/src/config.rs`
5. Add templates in `templates/your_module/`

## Adding a New CI Provider

1. Create `templates/ci/<provider>/ci.yml` (and cd.yml, etc.)
2. Add output path mapping in `crates/modules/src/ci.rs`

## Adding a New Agent Tool

1. Create template in `templates/agent/<tool-rules-file>`
2. Add generation branch in `crates/modules/src/agent.rs`

## Adding a New Language Preset

1. Add Makefile template: `templates/build/make/<language>`
2. Add .gitignore fragment in `crates/modules/src/git.rs`
3. Add linter config in `templates/quality/`
4. Add CI steps if needed

## Quality

```bash
cargo fmt        # Format
cargo clippy     # Lint
cargo test       # Test
```

All PRs must pass CI (fmt + clippy + test).
