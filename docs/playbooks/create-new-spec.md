# Playbook: Create a New Spec

## Steps

1. Find next SPEC-NNN in `docs/specs/_index.md`
2. `mkdir -p docs/specs/active/SPEC-NNN`
3. Copy templates: `cp docs/specs/_templates/{prd,technical-design}.md docs/specs/active/SPEC-NNN/`
4. Fill in PRD (problem, goals, user stories)
5. Fill in TD (API, implementation, tests)
6. Register in `_index.md` Active table

## Lifecycle

| Status | Meaning |
|--------|---------|
| Draft | Being written |
| Active | Implementation in progress |
| Completed | Verified by tests |
