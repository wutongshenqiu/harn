# Technical Design: Neutralize agent workflow SSOT and add first-class Codex overlays

| Field     | Value          |
|-----------|----------------|
| Spec ID   | SPEC-008       |
| Title     | Neutralize agent workflow SSOT and add first-class Codex overlays |
| Author    | Codex          |
| Status    | Completed      |
| Created   | 2026-03-24     |
| Updated   | 2026-03-24     |

## Overview

This spec introduces a tool-neutral workflow model for the agent module and uses it to generate
Claude, OpenCode, and Codex overlays from one canonical source.

The current implementation still exposes two architectural smells:

- `codex` is accepted in config but does not generate repo-local workflow artifacts.
- `.claude/commands/` is treated as the practical source of truth for workflows.

The fix is to split “workflow definition” from “tool-specific rendering”.

## API Design

### Generated Outputs

```text
AGENTS.md
CLAUDE.md
.claude/commands/<command>.md
.opencode/commands/<command>.md
.agents/skills/<command>/SKILL.md
.agents/codex/print-config.sh   # or equivalent helper
```

### Behavior

- `AGENTS.md` remains the repo-wide brief.
- `CLAUDE.md` remains a Claude-specific overlay.
- Claude, OpenCode, and Codex workflow artifacts are rendered from shared workflow definitions.
- Cursor, Windsurf, Cline, and Qoder remain rule-based overlays in this slice.

## Implementation

### Module Structure

```text
crates/
  core/src/
    agent_tools.rs         # richer artifact metadata for Codex
  modules/src/
    agent.rs               # workflow model + tool renderers
    project_checks.rs      # Codex validation
  modules/tests/
    template_coverage.rs   # workflow coverage tests
templates/
  agent/
    workflows/             # canonical workflow definitions
    codex/                 # Codex helper templates
    commands/              # compatibility layer or transitional wrappers
```

### Key Types

```rust
pub struct AgentWorkflow {
    pub id: &'static str,
    pub title: &'static str,
    pub body_template: &'static str,
}

pub enum AgentToolArtifact {
    File(&'static str),
    CommandsDir(&'static str),
    SkillsDir(&'static str),
}
```

### Flow

1. Load configured workflow ids from `AgentConfig.commands`.
2. Resolve each command id to a canonical workflow definition.
3. Render downstream overlays per tool:
   - Claude -> `.claude/commands/*.md`
   - OpenCode -> `.opencode/commands/*.md`
   - Codex -> `.agents/skills/*/SKILL.md`
4. Render shared top-level docs (`AGENTS.md`, `CLAUDE.md`).
5. Validate expected artifacts through `agent_tools.rs` and `project_checks.rs`.

## Configuration Changes

- No new top-level config fields required for the first slice.
- Generated Codex helper output may introduce a repo-local `.agents/codex/` path.

## Alternative Approaches

| Approach | Pros | Cons | Verdict |
|----------|------|------|---------|
| Keep `.claude/commands/` as SSOT | Minimal code churn | Preserves tool bias and keeps Codex awkward | Reject |
| Give each tool its own workflow source | Simple local generation | Guarantees drift | Reject |
| Neutral workflow source + tool adapters | Clear architecture, future-proof | Requires small refactor now | Accept |

## Task Breakdown

- [ ] Task 1: Add canonical workflow template location under `templates/agent/`
- [ ] Task 2: Refactor `agent.rs` to resolve workflows separately from tool renderers
- [ ] Task 3: Add Codex workflow generation under `.agents/skills/<command>/SKILL.md`
- [ ] Task 4: Add generated Codex setup helper output
- [ ] Task 5: Extend `AgentToolArtifact` and supported tool metadata for Codex
- [ ] Task 6: Extend `project_checks.rs` so Codex missing artifacts are reported
- [ ] Task 7: Update generated docs and workflow templates to reference the neutral model
- [ ] Task 8: Add template coverage and generation tests for Claude/OpenCode/Codex
- [ ] Task 9: Run `make check`

## Test Strategy

- **Unit tests:** workflow resolution, Codex artifact path expansion, tool metadata validation
- **Integration tests:** tempdir generation for `tools = ["codex"]` and
  `tools = ["claude", "opencode", "codex"]`
- **Manual verification:** inspect generated `.agents/skills/` output and helper instructions in a
  sample project

## Revision Log

| Date       | Section | Change | Reason |
|------------|---------|--------|--------|
| 2026-03-24 | All     | Initial design | Implement issues #32 and #33 |
| 2026-03-24 | Status  | Completed implementation and verification | Neutral workflow source, Codex overlays, and doctor coverage shipped |
