# PRD: Fix write pipeline, non-interactive issue, and doctor depth

| Field     | Value          |
|-----------|----------------|
| Spec ID   | SPEC-007       |
| Title     | Fix write pipeline, non-interactive issue, and doctor depth |
| Author    | Claude         |
| Status    | Active         |
| Created   | 2026-03-14     |
| Updated   | 2026-03-14     |

## Problem Statement

Three open issues (#24, #25, #26) expose fundamental gaps in harn's core:

1. **Write pipeline has no safety net** — `--force` silently destroys user customizations with no backup; `--dry-run` shows flat file list with no CREATE/OVERWRITE/SKIP distinction and no diffs.
2. **`harn issue` is interactive-only** — AI agents and CI cannot use it; contradicts harn's AI-first positioning.
3. **`harn doctor` checks are shallow** — Build doesn't validate targets, Git doesn't check .gitignore coverage, Quality doesn't check linter config, `pre_commit_hook` config is dead code.

No backward compatibility required — v0.1.0 has no external consumers.

## Goals

- `--force` creates `.harn-backup/` before overwriting any existing file
- `--dry-run` prints each file with status (CREATE / OVERWRITE / SKIP) and unified diffs for overwrites
- `harn issue` accepts `--type`, `--title`, `--body` for non-interactive use, plus `--open` for browser-only
- `pre_commit_hook` config is respected: when false, no hooks section in settings.json
- `harn doctor` validates Build targets, Git .gitignore coverage, Quality linter config presence

## Non-Goals

- `harn update` command (separate spec)
- `<!-- harn:managed -->` markers in generated files (separate spec)
- Three-way merge or template versioning (separate spec)

## User Stories

- As a CI pipeline, I want to run `harn issue --type bug --title "..." --body "..."` so that I can automate issue creation.
- As a developer using `--force`, I want my previous files backed up to `.harn-backup/` so that I can recover customizations.
- As a developer using `--dry-run`, I want to see which files would be created vs overwritten and what the diffs are, so that I can make informed decisions.
- As a developer with `pre_commit_hook = false`, I want no hooks generated in `.claude/settings.json`.
- As a developer running `harn doctor`, I want meaningful checks for Build targets, .gitignore coverage, and linter configs.

## Success Metrics

- `harn issue --type bug --title "test" --body "test"` creates an issue without interactive prompts
- `harn init --dry-run` output distinguishes CREATE / OVERWRITE / SKIP per file
- `harn add agent --force` creates `.harn-backup/` with copies of overwritten files
- `pre_commit_hook = false` in harn.toml results in no `hooks` key in generated settings.json
- `harn doctor` reports warnings for missing Makefile targets, incomplete .gitignore, missing linter configs

## Constraints

- All changes must pass `make check` (clippy pedantic, zero warnings, all tests)
- No new workspace dependencies unless strictly necessary
- Edition 2024, rust-version 1.87

## Open Questions

- None — all designs are straightforward.

## Design Decisions

| Decision | Options Considered | Chosen | Rationale |
|----------|--------------------|--------|-----------|
| Backup location | `.harn-backup/<timestamp>/` vs `.harn-backup/` flat | Flat `.harn-backup/` | Simpler; git provides history; backup is safety net not version control |
| write_file return type | `bool` vs `WriteStatus` enum | `WriteStatus` enum | No compat concern; enum carries CREATE/OVERWRITE/SKIP info cleanly |
| Dry-run diff | Full unified diff vs summary | Unified diff via manual impl | Avoid adding `similar` crate; simple line-by-line diff sufficient |
| Issue CLI fallback | All-or-nothing vs partial interactive | Partial interactive | If only some flags given, prompt for missing — composable UX |
