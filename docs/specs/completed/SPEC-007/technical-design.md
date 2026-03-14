# Technical Design: Fix write pipeline, non-interactive issue, and doctor depth

| Field     | Value          |
|-----------|----------------|
| Spec ID   | SPEC-007       |
| Title     | Fix write pipeline, non-interactive issue, and doctor depth |
| Author    | Claude         |
| Status    | Active         |
| Created   | 2026-03-14     |
| Updated   | 2026-03-14     |

## Overview

This spec fixes issues #24, #25, #26 by refactoring the write pipeline (backup + status-aware dry-run), adding non-interactive flags to `harn issue`, fixing `pre_commit_hook` dead code, and deepening `harn doctor` checks.

No backward compatibility constraints — all interfaces can change freely.

## Implementation

### 1. Write Pipeline Refactor

**`crates/core/src/context.rs`** — Replace `write_file` return type:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WriteStatus {
    Created,
    Overwritten,
    Skipped,
    WouldCreate,
    WouldOverwrite,
}

impl WriteStatus {
    pub fn is_written(self) -> bool {
        matches!(self, Self::Created | Self::Overwritten | Self::WouldCreate | Self::WouldOverwrite)
    }
}
```

`write_file(&self, path, content) -> io::Result<WriteStatus>`:
- `exists && !force` → `Skipped`
- `exists && force && dry_run` → `WouldOverwrite`
- `!exists && dry_run` → `WouldCreate`
- `exists && force && !dry_run` → backup then write → `Overwritten`
- `!exists && !dry_run` → write → `Created`

Backup logic: copy existing file to `root/.harn-backup/<relative_path>`.

**`crates/templates/src/engine.rs`** — Same pattern for `render_to` and `copy_to`:
- Add `backup_root: Option<PathBuf>` field
- Add `TemplateEngine::with_config(dry_run, backup_root)` constructor
- Before overwriting, backup to `backup_root/.harn-backup/<relative>`
- Return `WriteStatus` instead of `bool`

**`crates/modules/src/*.rs`** — Update all modules:
- Change `if engine.render_to(...)? {` to `if engine.render_to(...)?.is_written() {`
- Change `if ctx.write_file(...)? {` to `if ctx.write_file(...)?.is_written() {`

**`crates/cli/src/main.rs`** — Update `print_created_files`:
- Accept `Vec<(String, WriteStatus)>` instead of `Vec<String>`
- Print each status with distinct label and color
- For `WouldOverwrite`: read existing file, diff against rendered content, print unified diff
- Module `generate` returns `Vec<(String, WriteStatus)>` instead of `Vec<String>`

### 2. Non-Interactive `harn issue`

**`crates/cli/src/main.rs`** — Change `Issue` variant:

```rust
Issue {
    #[arg(long, value_parser = ["bug", "feature", "question"])]
    r#type: Option<String>,

    #[arg(long)]
    title: Option<String>,

    #[arg(long)]
    body: Option<String>,

    /// Open browser to new issue page instead of creating via gh CLI
    #[arg(long)]
    open: bool,
}
```

`cmd_issue` logic:
- If all three flags provided → non-interactive, skip dialoguer
- If partial → prompt only for missing fields
- `--open` → build URL with pre-filled params, open browser, return
- Support stdin: if stdin is not a terminal and body is None, read body from stdin

### 3. Fix `pre_commit_hook` Dead Code

**`crates/modules/src/agent.rs`** — `build_claude_settings`:
- Accept `&AgentConfig` parameter (currently only takes `&ProjectContext`)
- Check `agent_config.pre_commit_hook`
- If false, omit the `"hooks"` section from the generated JSON
- If true, generate hooks as before

### 4. Enhanced Doctor Checks

**`crates/modules/src/project_checks.rs`**:

**Build module** — `check_build`:
- Read Makefile/Justfile/Taskfile content
- Check for required targets: `build`, `test`, `lint`, `fmt`
- Warn for each missing target

**Git module** — `check_git`:
- Read .gitignore content
- For each language in `stacks.languages`, check for language-specific patterns:
  - rust: `/target/`
  - go: vendor or binary patterns
  - typescript/javascript: `node_modules/`
  - python: `__pycache__/`, `*.pyc`, `.venv/`
  - java: `build/`, `*.class`
  - dart: `.dart_tool/`, `build/`
- Warn for missing patterns

**Quality module** — `check_quality`:
- For each language, check expected linter config:
  - rust: clippy config in Cargo.toml or clippy.toml
  - typescript: `.eslintrc*` or `eslint.config.*`
  - python: `pyproject.toml` (ruff/black section) or `.flake8`
  - go: `.golangci.yml`
- Warn for missing configs

### Module Structure

```
crates/
  core/src/
    context.rs        # WriteStatus enum, backup logic
  templates/src/
    engine.rs         # backup_root field, WriteStatus return
  modules/src/
    agent.rs          # pre_commit_hook fix
    project_checks.rs # deeper Build/Git/Quality checks
    *.rs              # is_written() migration (mechanical)
  cli/src/
    main.rs           # Issue flags, print_created_files overhaul
```

### Key Types

```rust
// crates/core/src/context.rs
pub enum WriteStatus {
    Created, Overwritten, Skipped, WouldCreate, WouldOverwrite,
}

// Module trait return type change
pub trait Module {
    fn generate(&self, ctx: &mut ProjectContext) -> Result<Vec<(String, WriteStatus)>>;
}
```

## Configuration Changes

- No new config fields
- `.harn-backup/` directory created automatically when `--force` overwrites files
- `.harn-backup/` added to generated `.gitignore`

## Task Breakdown

- [ ] Task 1: Add `WriteStatus` enum to `crates/core/src/context.rs`, refactor `write_file`
- [ ] Task 2: Add backup logic to `context.rs` and `engine.rs`
- [ ] Task 3: Update `TemplateEngine` to return `WriteStatus`, add `backup_root`
- [ ] Task 4: Update `Module` trait return type to `Vec<(String, WriteStatus)>`
- [ ] Task 5: Migrate all 9 modules to new return type (mechanical `is_written()` change)
- [ ] Task 6: Update `print_created_files` with status labels, colors, and diff output
- [ ] Task 7: Add non-interactive flags to `harn issue`
- [ ] Task 8: Fix `pre_commit_hook` in `agent.rs` — respect config, pass `AgentConfig`
- [ ] Task 9: Enhanced `check_build` — validate required targets in Makefile
- [ ] Task 10: Enhanced `check_git` — validate .gitignore language coverage
- [ ] Task 11: Enhanced `check_quality` — validate linter config presence
- [ ] Task 12: Add `.harn-backup/` to generated `.gitignore`
- [ ] Task 13: Run `make check`, fix all clippy/test issues

## Test Strategy

- **Unit tests:** `WriteStatus::is_written()`, backup file creation, `check_build` target detection
- **Integration tests:** `harn init --dry-run` output format, `harn issue --type bug --title test --body test` exit code
- **Manual verification:** Run `harn add agent --force` on existing project, verify `.harn-backup/` created

## Revision Log

| Date       | Section | Change | Reason |
|------------|---------|--------|--------|
| 2026-03-14 | All     | Initial design | Issues #24, #25, #26 |
