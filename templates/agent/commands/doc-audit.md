Audit docs vs code consistency.

Steps:
1. Compare `docs/reference/api-conventions.md` with actual routes (skip if no API server)
2. Compare `docs/reference/data-model.md` with schema/migrations (skip if no database)
3. Compare `docs/reference/types/` with source code types (skip if no types defined)
4. Verify spec registry consistency (`docs/specs/_index.md` vs actual spec dirs)
5. Compare CLAUDE.md slash commands table with `.claude/commands/` files
6. Compare README feature claims with actual implementation
7. Report discrepancies table
