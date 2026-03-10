Orchestrate a long-running multi-spec plan. Argument $ARGUMENTS: `[create "title" SPEC-list | status | next | resume]`

## Overview

This command manages execution of multi-spec plans that span multiple context windows. It uses `.claude/current-plan.md` as persistent state and implements each spec sequentially.

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
3. Implement the spec: follow the spec's requirements and acceptance criteria
4. After implementation, verify with `make check`
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

## Implementation Notes

- Implement each spec directly in the main working tree
- One spec at a time — complete and verify before moving to the next
- If consecutive specs modify the same files, take extra care with ordering
