Audit docs vs code consistency.

Steps:
1. Compare `docs/reference/api-conventions.md` with actual routes (skip if no API server)
2. Compare `docs/reference/data-model.md` with schema/migrations (skip if no database)
3. Compare `docs/reference/types/` with source code types (skip if no types defined)
4. Verify spec registry consistency (`docs/specs/_index.md` vs actual spec dirs)
5. Compare `AGENTS.md` and `CLAUDE.md` against generated agent context
6. Compare `.agents/workflows/` against `.claude/commands/`, `.opencode/commands/`, and `.agents/skills/`
7. Compare README feature claims with actual implementation
8. Report discrepancies table
9. **Ask user which items to fix now** — present the actionable items and confirm before proceeding
