# {{ project_name }} — Agent Context

Universal context for AI coding agents. See CLAUDE.md for full reference.

## Commands

```bash
{{ build_tool }} build        # Build project
{{ build_tool }} test         # Run tests
{{ build_tool }} lint         # Run linters
{{ build_tool }} fmt          # Format code
{{ build_tool }} clean        # Clean artifacts
```

## Coding Rules

1. **Lint before commit** — `{{ build_tool }} lint` must pass
2. **Test before commit** — `{{ build_tool }} test` must pass
3. **No hardcoded secrets** — Use environment variables
4. **Conventional commits** — `feat:`, `fix:`, `docs:`, `refactor:`, `test:`, `chore:`
{% if project_type == "api" or project_type == "service" %}
5. **API envelope** — Standard response format
6. **SSOT** — Types in `docs/reference/types/`
7. **Doc sync** — Update reference docs when changing APIs/types/schema
{% endif %}

## Git Conventions

- Branches: `feat/xxx`, `fix/xxx`, `docs/xxx`, `refactor/xxx`
- Pre-commit: `{{ build_tool }} lint && {{ build_tool }} test`

## SDD (Spec-Driven Development)

```
Draft -> Active -> Completed
```

- Registry: `docs/specs/_index.md`
- Active: `docs/specs/active/`
- Completed: `docs/specs/completed/`
{% if project_type == "api" or project_type == "service" %}

## Reference (SSOT)

- Types: `docs/reference/types/`
- API: `docs/reference/api-conventions.md`
- Data Model: `docs/reference/data-model.md`
{% endif %}
- Playbooks: `docs/playbooks/`
