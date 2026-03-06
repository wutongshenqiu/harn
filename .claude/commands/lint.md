Run linting. Argument $ARGUMENTS: `[fix]`

Steps:
1. `make fmt` — Format code
2. `make lint` — Run linters
3. If `fix`: Auto-fix issues, re-run until clean
4. Report remaining issues with file:line references