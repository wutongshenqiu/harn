Manage Spec lifecycle. Argument $ARGUMENTS: subcommand.

Subcommands:
- `create "Title"` — Create new Spec in `docs/specs/active/`
- `list [active|completed|all]` — List Specs
- `status SPEC-NNN` — View Spec details
- `advance SPEC-NNN` — Advance: Draft -> Active -> Completed
- `complete SPEC-NNN` — Mark as completed: move dir from `active/` to `completed/`, update `_index.md` registry
- `td SPEC-NNN` — Create Technical Design from PRD

## `complete SPEC-NNN` details

1. Move `docs/specs/active/SPEC-NNN/` to `docs/specs/completed/SPEC-NNN/`
2. Update `docs/specs/_index.md`:
   - Remove from **Active** table
   - Add to **Completed** table with Status = "Completed" and Location = "completed/SPEC-NNN/"
3. If a plan is active (`.claude/current-plan.md` exists), update its status too
