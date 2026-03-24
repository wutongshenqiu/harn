Check CI status. Argument $ARGUMENTS: `[PR#]`

Steps:
1. If PR#: `gh pr checks <number>`, else: `gh run list --branch <branch>`
2. If failing: show `gh run view <id> --log-failed`
3. Report check status (pass/fail/pending)
