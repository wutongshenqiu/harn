# {{ project_name }}

## Overview

_Brief project description._

## Key Paths

```
TODO: List key directories and files
```

## Commands

```bash
make build        # Build project
make test         # Run tests
make lint         # Run linters
make fmt          # Format code
make clean        # Clean artifacts
```

## Slash Commands

| Command | Purpose |
|---------|---------|
{{ slash_commands_table }}

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
