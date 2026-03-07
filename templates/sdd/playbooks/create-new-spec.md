# Playbook: Create a New Spec

## Steps

1. Find next SPEC-NNN in `docs/specs/_index.md`
2. `mkdir -p docs/specs/active/SPEC-NNN`
3. Copy templates: `cp docs/specs/_templates/{prd,technical-design}.md docs/specs/active/SPEC-NNN/`
4. (Optional) If the spec requires competitor research or technical exploration, follow the `write-prd-td` playbook phases 1-2 before writing PRD/TD
5. Fill in PRD (problem, goals, user stories)
6. Fill in TD (API, implementation, tests)
7. Register in `_index.md` Active table

## Lifecycle

| Status | Meaning |
|--------|---------|
| Draft | Being written |
| Active | Implementation in progress |
| Completed | Verified by tests |

## Revising an Active Spec

When implementation reveals a design issue:

1. Update the relevant section in PRD or TD
2. Add an entry to TD's Revision Log with date and reason
3. Update Task Breakdown if tasks changed
4. Continue implementation from the updated spec
