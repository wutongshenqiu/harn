Diagnose and fix issues. Argument $ARGUMENTS: `[error description or file:line]`

Steps:
1. Parse error context from `$ARGUMENTS`
2. Gather context: read files, check logs, recent changes
3. Root cause analysis
4. Fix the issue
5. Verify: `make lint && make test`
6. Report: what went wrong, how it was fixed
