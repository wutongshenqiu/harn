# Technical Design: Add C/C++ Language Support

| Field     | Value          |
|-----------|----------------|
| Spec ID   | SPEC-004       |
| Title     | Add C/C++ Language Support |
| Author    | Claude          |
| Status    | Active         |
| Created   | 2026-03-06     |
| Updated   | 2026-03-06     |

## Overview

Add full C/C++ language support to harn, covering all language-aware modules: build templates, gitignore, quality config, agent permissions, and Docker template.

## Implementation

### Files to Create

**Build Templates** (3 files):
- `templates/build/make/cpp` — CMake-based build
- `templates/build/just/cpp`
- `templates/build/task/cpp`

Standard targets wrapping CMake: `dev`, `build`, `test`, `lint`, `fmt`, `clean`, `help`

**Quality Templates** (1 file):
- `templates/quality/clang-format` — `.clang-format` config (LLVM style base)

**Docker Templates** (1 file):
- `templates/docker/Dockerfile.cpp` — Multi-stage build with GCC/CMake

### Files to Modify

**git.rs** — Add `"cpp" | "c"` match arm:
```rust
"cpp" | "c" => {
    content.push_str("# C/C++\nbuild/\ncmake-build-*/\n*.o\n*.a\n*.so\n*.dylib\n*.exe\ncompile_commands.json\n\n");
}
```

**quality.rs** — Add `"cpp" | "c"` match arm:
```rust
"cpp" | "c" => {
    let src = "quality/clang-format";
    if engine.has_template(src) {
        let dst = ctx.path(".clang-format");
        if engine.render_to(src, &vars, &dst, force)? {
            created.push(".clang-format".into());
        }
    }
}
```

**agent.rs** — Add `"cpp" | "c"` match arm:
```rust
"cpp" | "c" => {
    perms.push("Bash(cmake:*)".into());
    perms.push("Bash(make:*)".into());
    perms.push("Bash(gcc:*)".into());
    perms.push("Bash(g++:*)".into());
    perms.push("Bash(clang:*)".into());
    perms.push("Bash(clang++:*)".into());
}
```

## Task Breakdown

- [ ] Create `templates/build/make/cpp`
- [ ] Create `templates/build/just/cpp`
- [ ] Create `templates/build/task/cpp`
- [ ] Create `templates/quality/clang-format`
- [ ] Create `templates/docker/Dockerfile.cpp`
- [ ] Update `crates/modules/src/git.rs` — add C/C++ gitignore
- [ ] Update `crates/modules/src/quality.rs` — add C/C++ match arm
- [ ] Update `crates/modules/src/agent.rs` — add C/C++ permissions
- [ ] Run `make check` to verify

## Test Strategy

- **Unit tests:** `make test` — ensure no regressions
- **Manual verification:** Run `cargo run -- init` with `languages = ["cpp"]` and verify all generated files
