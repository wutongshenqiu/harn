# {{ project_name }}

## Overview

_Brief project description._

## Key Paths

```
TODO: List key directories and files
```

## Commands

```bash
{{ build_tool }} build        # Build project
{{ build_tool }} test         # Run tests
{{ build_tool }} lint         # Run linters
{{ build_tool }} fmt          # Format code
{{ build_tool }} clean        # Clean artifacts
```

## Slash Commands

| Command | Purpose |
|---------|---------|
{{ slash_commands_table }}

## Coding Rules

1. **Lint before commit** — `{{ build_tool }} lint` must pass
2. **Test before commit** — `{{ build_tool }} test` must pass
{% if project_type == "api" or project_type == "service" %}
3. **API envelope** — Standard response format
4. **SSOT** — Types in `docs/reference/types/`
{% endif %}
5. **No hardcoded secrets** — Use env vars
6. **Conventional commits** — `feat:`, `fix:`, `docs:`, `refactor:`, `test:`, `chore:`
{% if project_type == "api" or project_type == "service" %}
7. **Doc sync** — Update reference docs when changing APIs/types/schema
{% endif %}

## Git Conventions

- Branches: `feat/xxx`, `fix/xxx`, `docs/xxx`, `refactor/xxx`
- Pre-commit: `{{ build_tool }} lint && {{ build_tool }} test`

## SDD (Spec-Driven Development)

```
Draft -> Active -> Completed
```

- Specs: `docs/specs/` (registry: `_index.md`)
{% if project_type == "api" or project_type == "service" %}
- Reference: `docs/reference/` (SSOT)
{% endif %}
- Playbooks: `docs/playbooks/`
