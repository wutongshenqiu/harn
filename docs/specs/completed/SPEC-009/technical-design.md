# Technical Design: Detect drift and prune stale generated agent overlays

| Field     | Value          |
|-----------|----------------|
| Spec ID   | SPEC-009       |
| Title     | Detect drift and prune stale generated agent overlays |
| Author    | Codex          |
| Status    | Completed      |
| Created   | 2026-03-24     |
| Updated   | 2026-03-24     |

## Overview

This spec adds explicit tracking, drift detection, and stale artifact cleanup for generated agent
overlays after SPEC-008 establishes a neutral workflow source model.

The key rule is simple: harn should know which overlay files it generated, validate that they still
match the canonical source, and remove obsolete ones only inside the managed overlay surface.

## API Design

### Managed Manifest

```json
{
  "version": 1,
  "artifacts": [
    ".claude/commands/review.md",
    ".opencode/commands/review.md",
    ".agents/skills/review/SKILL.md"
  ]
}
```

### Affected Commands

```text
harn init
harn add agent
harn doctor
harn doctor --fix
```

## Implementation

### Module Structure

```text
crates/
  modules/src/
    agent.rs               # emit manifest entries during generation
    project_checks.rs      # drift detection + stale artifact checks
  core/src/
    context.rs             # safe delete / backup helpers if needed
templates/
  agent/
    commands/
      doc-audit.md         # multi-tool overlay audit
      sync-commands.md     # neutral workflow source guidance
```

### Key Types

```rust
pub struct AgentOverlayManifest {
    pub version: u32,
    pub artifacts: Vec<String>,
}

pub enum AgentOverlayStatus {
    Missing,
    Drifted,
    Stale,
}
```

### Flow

1. Agent generation computes the full set of managed overlay paths for the configured tools and
   commands.
2. The set is written to a manifest such as `.harn/agent-overlays.json`.
3. Generation compares the previous manifest to the new one:
   - paths present before but not now become stale
   - stale managed files are removed, or reported in `--dry-run`
4. `harn doctor` loads the manifest, re-renders expected overlay content, and flags:
   - missing managed artifacts
   - drifted artifacts whose content no longer matches the canonical source
   - stale artifacts that still exist on disk but are not in the current manifest
5. `doc-audit` and `sync-commands` are updated to reason about neutral workflow sources and all
   generated overlay targets.

## Configuration Changes

- New internal manifest file under `.harn/`
- No new user-facing config required for the first implementation

## Alternative Approaches

| Approach | Pros | Cons | Verdict |
|----------|------|------|---------|
| Recompute everything with no manifest | Fewer files | Harder to distinguish stale vs expected outputs | Reject |
| Delete whole overlay directories on every run | Simple implementation | Unsafe for mixed user-managed content | Reject |
| Manifest + managed-path pruning | Safe, explicit, testable | Slightly more code | Accept |

## Task Breakdown

- [ ] Task 1: Define manifest format and path for managed agent overlays
- [ ] Task 2: Emit manifest content from the agent generation path
- [ ] Task 3: Add stale artifact diffing between previous and current manifests
- [ ] Task 4: Add safe stale artifact pruning with dry-run reporting
- [ ] Task 5: Extend `project_checks.rs` with missing/drifted/stale overlay diagnostics
- [ ] Task 6: Extend `doc-audit.md` and `sync-commands.md` to cover Claude/OpenCode/Codex overlays
- [ ] Task 7: Add regression tests for modified, missing, and stale overlay files
- [ ] Task 8: Verify `doctor --fix` behavior or explicitly defer it
- [ ] Task 9: Run `make check`

## Test Strategy

- **Unit tests:** manifest diffing, stale path classification, content drift detection
- **Integration tests:** tempdir scenario where commands/tools change and stale overlays are pruned
- **Manual verification:** run generation twice with different tool sets and inspect dry-run / doctor
  output

## Revision Log

| Date       | Section | Change | Reason |
|------------|---------|--------|--------|
| 2026-03-24 | All     | Initial design | Implement issue #34 |
| 2026-03-24 | Status  | Completed implementation and verification | Manifest-based drift detection and stale pruning shipped |
