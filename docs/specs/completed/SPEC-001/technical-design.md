# Technical Design: Complete Python Language Support

| Field     | Value          |
|-----------|----------------|
| Spec ID   | SPEC-001       |
| Title     | Complete Python Language Support |
| Author    | Claude          |
| Status    | Active         |
| Created   | 2026-03-06     |
| Updated   | 2026-03-06     |

## Overview

Python currently has limited support in harn: only gitignore fragments and agent permissions exist. This spec adds full support including build templates (Make/Just/Task), quality configs (ruff, pyproject.toml), and Docker templates.

## Implementation

### Files to Create

**Build Templates** (3 files):
- `templates/build/make/python`
- `templates/build/just/python`
- `templates/build/task/python`

Standard targets: `dev`, `build`, `test`, `lint`, `fmt`, `clean`, `help`
- Use `uv` as the modern Python package manager
- Use `ruff` for linting and formatting
- Use `pytest` for testing

**Quality Templates** (1 file):
- `templates/quality/ruff.toml` — Ruff linter/formatter config

### Files to Modify

**quality.rs** — Add `"python"` match arm:
- Generate `ruff.toml` from `quality/ruff.toml` template

### Module Changes

```rust
// In quality.rs, add to the match block:
"python" => {
    let src = "quality/ruff.toml";
    if engine.has_template(src) {
        let dst = ctx.path("ruff.toml");
        if engine.render_to(src, &vars, &dst, force)? {
            created.push("ruff.toml".into());
        }
    }
}
```

## Task Breakdown

- [ ] Create `templates/build/make/python` with uv + ruff + pytest targets
- [ ] Create `templates/build/just/python`
- [ ] Create `templates/build/task/python`
- [ ] Create `templates/quality/ruff.toml`
- [ ] Update `crates/modules/src/quality.rs` — add Python match arm
- [ ] Run `make check` to verify

## Test Strategy

- **Unit tests:** `make test` — ensure no regressions
- **Manual verification:** Run `cargo run -- init` with `languages = ["python"]` and verify generated files
