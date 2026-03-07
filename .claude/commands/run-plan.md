Orchestrate a long-running multi-spec plan. Argument $ARGUMENTS: `[create "title" SPEC-list [--full] | status | next | resume]`

## Overview

This command manages execution of multi-spec plans that span multiple context windows. It uses `.claude/current-plan.md` as persistent state and delegates each phase to the appropriate subagent.

## Phase Types

Plans support three phase types, each delegated to a specialized subagent:

| Phase Type | Subagent | Purpose |
|------------|----------|---------|
| `research:` | `spec-researcher` | Competitor research or technical feasibility analysis |
| `write:` | `spec-writer` | PRD or TD authoring based on research artifacts |
| `implement:` | `spec-implementer` | Code implementation from spec |

## Subcommands

### `create "title" SPEC-001,SPEC-002,...`

1. Parse the spec list from $ARGUMENTS
2. For each spec, verify it exists in `docs/specs/active/`
3. Create `.claude/current-plan.md` with structure:

```markdown
# Plan: [title]

## Phases
- [ ] implement: SPEC-001 — [title from spec]
- [ ] implement: SPEC-002 — [title from spec]

## Progress Log
[empty initially]
```

4. Report: plan created, ready to run with `/run-plan next`

### `create "title" SPEC-NNN --full`

Creates a plan with the full lifecycle (research + write + implement) for a single spec:

1. Verify spec exists in `docs/specs/active/`
2. Create `.claude/current-plan.md` with structure:

```markdown
# Plan: [title]

## Phases
- [ ] research: SPEC-NNN competitor research → research-competitors.md
- [ ] research: SPEC-NNN technical feasibility → research-tech.md
- [ ] write: SPEC-NNN PRD → prd.md
- [ ] write: SPEC-NNN TD → technical-design.md
- [ ] implement: SPEC-NNN → code

## Progress Log
[empty initially]
```

3. Report: plan created with 5 phases, ready to run with `/run-plan next`

### `status`

1. Read `.claude/current-plan.md`
2. Report: completed specs, current spec, remaining specs

### `next`

1. Read `.claude/current-plan.md`
2. Find the first unchecked phase
3. Delegate to the appropriate subagent based on phase type:
   - `research:` → `spec-researcher` subagent: "Research [topic] for [SPEC-NNN]"
   - `write:` → `spec-writer` subagent: "Write [prd/td] for [SPEC-NNN]"
   - `implement:` → `spec-implementer` subagent: "Implement [SPEC-NNN]"
4. When subagent returns, parse its summary
5. Update `.claude/current-plan.md`:
   - Mark phase as `[x]`
   - Append to Progress Log: timestamp, phase type, spec id, status, output file(s)
6. If partial completion, note it and ask user whether to retry or continue
7. If all phases done:
   - Report completion summary
   - Archive plan: rename `.claude/current-plan.md` → `.claude/plans/completed-{title-slug}-{date}.md`
   - Suggest: `/ship` to create PR

### `resume`

Same as `next` — reads current-plan.md and continues from where it left off.

## Important

- Always read `.claude/current-plan.md` before any action — it is the source of truth
- Always update `.claude/current-plan.md` after each phase completes — it survives context compression
- One phase at a time — do not parallelize phase execution
- After `implement:` phases, verify the build still passes: `make check`
- `research:` and `write:` phases do not need `make check` — they produce documents, not code

## Worktree vs Direct Implementation

- **Use worktree isolation** (default) when specs touch independent files — maximizes parallelism and safety
- **Skip worktree isolation** when consecutive specs modify the same source files (e.g., multiple languages all adding match arms to the same `.rs` files) — avoids merge conflicts
- To skip worktree: implement directly in the main working tree instead of delegating to `spec-implementer` with `isolation: "worktree"`
- Signs you should skip worktree: specs that all touch `git.rs`, `quality.rs`, `agent.rs`, or other shared modules
