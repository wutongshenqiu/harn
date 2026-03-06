# {{ project_name }}

## Overview

_Brief project description._

## Key Paths

```
TODO: List key directories and files
```

## Commands

```bash
make dev          # Start development
make build        # Build project
make test         # Run tests
make lint         # Run linters
make fmt          # Format code
make clean        # Clean artifacts
```

## Slash Commands

| Command | Purpose |
|---------|---------|
| `/ship [msg]` | Lint + test + commit + push + PR |
| `/implement SPEC-NNN` | Implement a spec |
| `/spec create/list/advance` | Manage spec lifecycle |
| `/lint [fix]` | Run linters |
| `/test [scope]` | Run tests |
| `/review [PR#]` | Code review |
| `/diagnose [error]` | Diagnose issues |
| `/deps [check/update]` | Manage dependencies |
| `/doc-audit` | Audit docs vs code |

## Coding Rules

1. **Lint before commit** — `make lint` must pass
2. **Test before commit** — `make test` must pass
3. **API envelope** — Standard response format
4. **SSOT** — Types in `docs/reference/types/`
5. **No hardcoded secrets** — Use env vars
6. **Conventional commits** — `feat:`, `fix:`, `docs:`, `refactor:`, `test:`, `chore:`
7. **Doc sync** — Update reference docs when changing APIs/types/schema

## Git Conventions

- Branches: `feat/xxx`, `fix/xxx`, `docs/xxx`, `refactor/xxx`
- Pre-commit: `make lint && make test`

## SDD (Spec-Driven Development)

```
Draft -> Active -> Completed
```

- Specs: `docs/specs/` (registry: `_index.md`)
- Reference: `docs/reference/` (SSOT)
- Playbooks: `docs/playbooks/`
