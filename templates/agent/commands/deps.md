Dependency management. Argument $ARGUMENTS: `[check|update|add|audit]`

Steps:
1. Parse subcommand
2. Execute: check outdated, update, add package, or security audit
3. Verify: `make lint && make test`
4. Report changes
