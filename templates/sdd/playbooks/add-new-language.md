# Playbook: Add a New Language

How to add full support for a new programming language to harn.

## Touchpoints (6 areas)

### 1. Build Templates

Create 3 template files with standard targets (`dev`, `build`, `test`, `lint`, `fmt`, `clean`, `help`):

```
templates/build/make/<lang>
templates/build/just/<lang>
templates/build/task/<lang>
```

Reference existing templates (e.g., `templates/build/make/go`) for style.

### 2. Git Ignore — `crates/modules/src/git.rs`

Add a match arm in the `for lang in &ctx.config.stacks.languages` block:

```rust
"<lang>" => {
    content.push_str("# <Lang>\n<patterns>\n\n");
}
```

### 3. Quality Config — `crates/modules/src/quality.rs` + template

Create the linter config template:

```
templates/quality/<config-file>
```

Add a match arm in quality.rs to render it:

```rust
"<lang>" => {
    let src = "quality/<config-file>";
    if engine.has_template(src) {
        let dst = ctx.path("<output-file>");
        if engine.render_to(src, &vars, &dst, force)? {
            created.push("<output-file>".into());
        }
    }
}
```

### 4. Agent Permissions — `crates/modules/src/agent.rs`

Add a match arm in `build_claude_settings()` to grant tool permissions:

```rust
"<lang>" => {
    perms.push("Bash(<compiler>:*)".into());
    perms.push("Bash(<package-mgr>:*)".into());
}
```

### 5. Docker Template

Create a Dockerfile template:

```
templates/docker/Dockerfile.<lang>
```

No code changes needed — `docker.rs` auto-discovers `Dockerfile.<lang>`.

### 6. Documentation

Update these files:
- `README.md` — Language Support table
- `CLAUDE.md` — already references this playbook in Extension Points

## Verification

```bash
make check    # Must pass (fmt + clippy + tests)
```

## Existing Languages

| Language | Build Tool | Linter | Key Patterns |
|----------|-----------|--------|--------------|
| rust | cargo | rust-toolchain.toml | target/ |
| go | go | .golangci.yml | bin/, coverage.out |
| typescript | npm | eslint + prettier | node_modules/, dist/ |
| dart | flutter | — | .dart_tool/, build/ |
| python | uv | ruff.toml | \_\_pycache\_\_/, .venv/ |
| java | gradle | checkstyle.xml | build/, .gradle/, *.class |
| cpp / c | cmake | .clang-format | build/, *.o, *.a |
