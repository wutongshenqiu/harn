# PRD: Neutralize agent workflow SSOT and add first-class Codex overlays

| Field     | Value          |
|-----------|----------------|
| Spec ID   | SPEC-008       |
| Title     | Neutralize agent workflow SSOT and add first-class Codex overlays |
| Author    | Codex          |
| Status    | Completed      |
| Created   | 2026-03-24     |
| Updated   | 2026-03-24     |

## Problem Statement

The current agent module still treats Claude-oriented command templates as the canonical workflow
source, while Codex support is effectively limited to `AGENTS.md`.

This causes three concrete problems:

1. Codex is listed as supported, but generation is still a no-op branch in the agent module.
2. Workflow semantics are tied to `.claude/commands/`, which makes OpenCode and Codex overlays
   look like mirrors instead of first-class outputs.
3. Doctor and docs can only partially validate or describe the generated workflow layer.

This spec addresses the architectural gap behind issues #32 and #33.

## Goals

- Introduce a tool-neutral workflow source model for agent workflows.
- Generate Claude, OpenCode, and Codex overlays from the same canonical workflow definitions.
- Make Codex produce repo-local artifacts beyond `AGENTS.md`.
- Extend doctor so Codex has meaningful expected artifacts and validation.
- Update generated docs so `AGENTS.md` is the repo-wide brief and tool overlays are downstream views.

## Non-Goals

- Drift detection and stale artifact pruning across generated overlays.
- Full parity upgrades for Cursor, Windsurf, Cline, or Qoder beyond their current rule files.
- Merge tooling for user-edited generated files.

## User Stories

- As a team using Codex, I want harn to generate repo-local Codex workflow artifacts so that the
  configured `codex` tool is actually usable.
- As a maintainer, I want workflow definitions to live in a tool-neutral location so that new tools
  do not inherit Claude-specific assumptions.
- As a developer running `harn doctor`, I want Codex artifacts validated like other configured
  tools so that missing setup is caught early.

## Success Metrics

- `tools = ["codex"]` generates Codex-specific repo artifacts in addition to `AGENTS.md`.
- `tools = ["claude", "opencode", "codex"]` generates all three overlay sets from a shared
  workflow source.
- `harn doctor` warns when required Codex overlay artifacts are missing.
- The workflow SSOT no longer points to `.claude/commands/` in generated workflow docs.

## Constraints

- Preserve existing Claude and OpenCode output behavior for default commands.
- Keep the implementation within the current Rust workspace and template engine model.
- Pass `make check` and add regression coverage for generation and doctor validation.

## Open Questions

- [ ] Should the Codex helper output be a generated README, a script that prints config stanzas, or
      both?

## Design Decisions

| Decision | Options Considered | Chosen | Rationale |
|----------|--------------------|--------|-----------|
| Workflow SSOT location | Keep `templates/agent/commands/` vs add neutral location | Add neutral workflow location | Removes Claude bias and gives Codex a canonical upstream source |
| Codex artifact shape | `AGENTS.md` only vs repo-local skills | Repo-local skills under `.agents/skills/` | Matches Codex workflow expectations and gives doctor concrete artifacts |
| Scope of first-class support | Docs-only vs generated overlays + validation | Generated overlays + validation | Avoids repeating the current “supported but mostly no-op” state |
