End-to-end commit pipeline. Argument $ARGUMENTS: `[--no-pr] [--merge] [commit message]`

Steps:

1. **Parse arguments**: Extract flags and commit message from `$ARGUMENTS`
2. **Existing PR check**: `gh pr list --head <branch> --state open`
3. **Commands sync check**: Diff `.claude/commands/` vs `templates/agent/commands/` — warn if any drift detected
4. **Format + Lint**: `make fmt && make lint` — fix issues until pass
4. **Test**: `make test` — fix failures until pass
5. **SDD health check**: If `docs/specs/` exists, run `harn doctor` — fix errors before proceeding (warnings are OK)
6. **Doc sync check**: If API/types/schema changed, verify reference docs updated
7. **Spec check**: If changes complete an Active Spec, advance to Completed
8. **Branch management**: If on `main`, create feature branch from commit message
9. **Stage**: `git add` (exclude `.env`, secrets)
10. **Commit**: Conventional commit format
11. **Push**: `git push -u origin HEAD`
12. **Create PR** (unless `--no-pr`): Title + Summary + Test Plan
13. **Watch CI**: Wait for checks to pass
14. **Merge** (if `--merge`): Auto-merge after CI passes
15. **Report**: Commit SHA, PR URL, merge status
