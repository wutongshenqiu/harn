# PRD: Detect drift and prune stale generated agent overlays

| Field     | Value          |
|-----------|----------------|
| Spec ID   | SPEC-009       |
| Title     | Detect drift and prune stale generated agent overlays |
| Author    | Codex          |
| Status    | Completed      |
| Created   | 2026-03-24     |
| Updated   | 2026-03-24     |

## Problem Statement

Once multiple agent overlays exist, harn has no reliable way to detect whether generated artifacts
have drifted from their source definitions or whether stale files should be removed.

The current gaps are:

1. Doctor checks for existence, but not freshness, for generated agent overlays.
2. Sync guidance still assumes a narrow Claude/OpenCode world and does not reason about the full
   overlay graph.
3. Stale generated files and directories are never pruned, which lets dead workflows linger after
   config changes.

This spec addresses issue #34 after the tool-neutral workflow model is in place.

## Goals

- Detect drift between canonical workflow definitions and generated downstream overlays.
- Track harn-managed agent overlay artifacts explicitly.
- Prune stale generated overlay files and directories safely.
- Extend doctor and doc-audit so they validate the full multi-tool overlay set.

## Non-Goals

- Three-way merge for user modifications inside managed overlay files.
- Drift detection for unrelated modules outside the agent workflow layer.
- Deleting user-owned files outside harn-managed overlay paths.

## User Stories

- As a maintainer, I want `harn doctor` to tell me when generated agent overlays are stale so that
  configuration drift does not go unnoticed.
- As a developer changing `modules.agent.tools` or `commands`, I want old generated overlays
  removed automatically so that the repo only contains live workflow artifacts.
- As a team using multiple tools, I want doc-audit to compare all relevant overlays so that docs and
  generation stay aligned.

## Success Metrics

- `harn doctor` reports drift when a generated overlay file differs from the canonical workflow
  source.
- Removing a command from config causes stale downstream overlays to be reported and pruned.
- `doc-audit` covers `AGENTS.md`, `CLAUDE.md`, Claude/OpenCode command files, and Codex skills.
- Pruning is limited to harn-managed overlay paths and does not touch unrelated files.

## Constraints

- Reuse the existing write/backup safety model where practical.
- Keep pruning deterministic and reviewable in `--dry-run`.
- Add tests for missing, modified, and stale artifact scenarios.

## Open Questions

- [ ] Should stale overlay deletion be offered only via generation/sync, or also via `doctor --fix`?

## Design Decisions

| Decision | Options Considered | Chosen | Rationale |
|----------|--------------------|--------|-----------|
| Artifact tracking | Deterministic recompute only vs generated manifest | Generated manifest | Simplifies drift/prune decisions and dry-run output |
| Prune scope | Entire tool dirs vs manifest-owned paths only | Manifest-owned paths only | Safer and compatible with user custom files beside managed output |
| Drift comparison | Existence-only vs byte-for-byte content checks | Byte-for-byte checks | Detects real freshness problems, not just missing files |
