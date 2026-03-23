# {{ project_name }} — Claude Code Context

Read `AGENTS.md` first. This file adds Claude Code specific workflow details.

## Claude Files

- `.claude/settings.json` — tool permissions and hooks
- `.claude/commands/` — slash command implementations
- `AGENTS.md` — repo-wide coding rules and project context

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

1. Use `AGENTS.md` as the repo-wide source of truth
2. Keep `.claude/commands/` aligned with the slash command table below
3. Review `.claude/settings.json` permissions before enabling broad shell access
4. Run `{{ build_tool }} lint` and `{{ build_tool }} test` before shipping

## Workflow

- Start from `AGENTS.md` for project structure, rules, and references
- Use slash commands from `.claude/commands/` for Claude-specific workflows
- Update `AGENTS.md` when the persistent project context changes
