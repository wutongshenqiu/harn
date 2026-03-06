Run tests. Argument $ARGUMENTS: `[unit|integration|e2e|all] [filter]`

Steps:
1. Parse scope from `$ARGUMENTS`
2. Run `make test` (or specific target)
3. Analyze failures
4. Report pass/fail summary
