# Technical Design: Add Java Language Support

| Field     | Value          |
|-----------|----------------|
| Spec ID   | SPEC-003       |
| Title     | Add Java Language Support |
| Author    | Claude          |
| Status    | Active         |
| Created   | 2026-03-06     |
| Updated   | 2026-03-06     |

## Overview

Add full Java language support to harn, covering all language-aware modules: build templates, gitignore, quality config, agent permissions, and Docker template.

## Implementation

### Files to Create

**Build Templates** (3 files):
- `templates/build/make/java` — Gradle-based (default Java build tool)
- `templates/build/just/java`
- `templates/build/task/java`

Standard targets wrapping Gradle: `dev`, `build`, `test`, `lint`, `fmt`, `clean`, `help`

**Quality Templates** (1 file):
- `templates/quality/checkstyle.xml` — Checkstyle config (Google style)

**Docker Templates** (1 file):
- `templates/docker/Dockerfile.java` — Multi-stage build with Eclipse Temurin JDK

### Files to Modify

**git.rs** — Add `"java"` match arm:
```rust
"java" => {
    content.push_str("# Java\nbuild/\n.gradle/\n*.class\n*.jar\n*.war\nout/\n\n");
}
```

**quality.rs** — Add `"java"` match arm:
```rust
"java" => {
    let src = "quality/checkstyle.xml";
    if engine.has_template(src) {
        let dst = ctx.path("checkstyle.xml");
        if engine.render_to(src, &vars, &dst, force)? {
            created.push("checkstyle.xml".into());
        }
    }
}
```

**agent.rs** — Add `"java"` match arm:
```rust
"java" => {
    perms.push("Bash(java:*)".into());
    perms.push("Bash(javac:*)".into());
    perms.push("Bash(gradle:*)".into());
    perms.push("Bash(./gradlew:*)".into());
    perms.push("Bash(mvn:*)".into());
}
```

## Task Breakdown

- [ ] Create `templates/build/make/java`
- [ ] Create `templates/build/just/java`
- [ ] Create `templates/build/task/java`
- [ ] Create `templates/quality/checkstyle.xml`
- [ ] Create `templates/docker/Dockerfile.java`
- [ ] Update `crates/modules/src/git.rs` — add Java gitignore
- [ ] Update `crates/modules/src/quality.rs` — add Java match arm
- [ ] Update `crates/modules/src/agent.rs` — add Java permissions
- [ ] Run `make check` to verify

## Test Strategy

- **Unit tests:** `make test` — ensure no regressions
- **Manual verification:** Run `cargo run -- init` with `languages = ["java"]` and verify all generated files
