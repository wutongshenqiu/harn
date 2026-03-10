Sync commands across tools. Argument $ARGUMENTS: `[command-name | all]`

SSOT: `.claude/commands/` -> `.cursor/rules/`, `.qoder/rules/`, `.opencode/commands/`

Steps:
1. Determine scope from `$ARGUMENTS`
2. Compare definitions across tool locations
3. Generate updates (SKILL.md format, OpenCode format)
4. Write divergent files
5. Report sync results
