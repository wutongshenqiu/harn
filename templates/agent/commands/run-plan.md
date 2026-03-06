Orchestrate a long-running multi-spec plan. Argument $ARGUMENTS: `[create "title" SPEC-list | status | next | resume]`

## Overview

This command manages execution of multi-spec plans that span multiple context windows. It uses `.claude/current-plan.md` as persistent state and delegates each spec to the `spec-implementer` subagent.

## Subcommands

### `create "title" SPEC-001,SPEC-002,...`

1. Parse the spec list from $ARGUMENTS
2. For each spec, verify it exists in `docs/specs/active/`
3. Create `.claude/current-plan.md` with structure:

```markdown
# Plan: [title]

## Specs
- [ ] SPEC-001 — [title from spec]
- [ ] SPEC-002 — [title from spec]

## Progress Log
[empty initially]
```

4. Report: plan created, ready to run with `/run-plan next`

### `status`

1. Read `.claude/current-plan.md`
2. Report: completed specs, current spec, remaining specs

### `next`

1. Read `.claude/current-plan.md`
2. Find the first unchecked spec
3. Delegate to `spec-implementer` subagent: "Implement [SPEC-NNN]"
4. When subagent returns, parse its summary
5. Update `.claude/current-plan.md`:
   - Mark spec as `[x]`
   - Append to Progress Log: timestamp, spec id, status, files modified, commits
6. If partial completion, note it and ask user whether to retry or continue
7. If all specs done:
   - Report completion summary
   - Suggest: `/ship` to create PR

### `resume`

Same as `next` — reads current-plan.md and continues from where it left off.

## Important

- Always read `.claude/current-plan.md` before any action — it is the source of truth
- Always update `.claude/current-plan.md` after each spec completes — it survives context compression
- One spec at a time — do not parallelize spec implementation
- After each spec, verify the build still passes: `make check`

## Worktree vs Direct Implementation

- **Use worktree isolation** (default) when specs touch independent files — maximizes parallelism and safety
- **Skip worktree isolation** when consecutive specs modify the same source files (e.g., multiple languages all adding match arms to the same `.rs` files) — avoids merge conflicts
- To skip worktree: implement directly in the main working tree instead of delegating to `spec-implementer` with `isolation: "worktree"`
- Signs you should skip worktree: specs that all touch `git.rs`, `quality.rs`, `agent.rs`, or other shared modules
