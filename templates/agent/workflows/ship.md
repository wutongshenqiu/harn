End-to-end commit pipeline. Argument $ARGUMENTS: `[--no-pr] [--merge] [commit message]`

Steps:

1. **Parse arguments**: Extract flags and commit message from `$ARGUMENTS`
2. **Existing PR check**: `gh pr list --head <branch> --state open`
3. **Format + Lint**: `make fmt && make lint` — fix issues until pass
4. **Test**: `make test` — fix failures until pass
5. **Doc sync check**: If API/types/schema changed, verify reference docs updated
6. **Spec check**: If changes complete an Active Spec, advance to Completed
7. **Branch management**: If on `main`, create feature branch from commit message
8. **Stage**: `git add` (exclude `.env`, secrets)
9. **Commit**: Conventional commit format
10. **Push**: `git push -u origin HEAD`
11. **Create PR** (unless `--no-pr`): Title + Summary + Test Plan
12. **Watch CI**: Wait for checks to pass
13. **Merge** (if `--merge`): Auto-merge after CI passes
14. **Report**: Commit SHA, PR URL, merge status
