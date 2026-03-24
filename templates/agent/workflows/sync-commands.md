Sync commands across tools. Argument $ARGUMENTS: `[command-name | all]`

SSOT: `.agents/workflows/` -> `.claude/commands/`, `.opencode/commands/`, `.agents/skills/`

Steps:
1. Determine scope from `$ARGUMENTS`
2. Compare `.agents/workflows/` against all generated downstream overlays
3. Regenerate divergent Claude / OpenCode / Codex files from the workflow source
4. Report stale overlays that should be pruned
5. Report sync results
