#!/bin/bash
# Injects current plan state into Claude's context.
# Used by UserPromptSubmit and SessionStart(compact) hooks.
# Only outputs when a plan file exists, keeping normal sessions clean.

PLAN_FILE="$CLAUDE_PROJECT_DIR/.claude/current-plan.md"

if [ -f "$PLAN_FILE" ]; then
  echo "=== ACTIVE PLAN (from .claude/current-plan.md) ==="
  head -50 "$PLAN_FILE"
  TOTAL=$(wc -l < "$PLAN_FILE")
  if [ "$TOTAL" -gt 50 ]; then
    echo "... (truncated, $TOTAL lines total. Read .claude/current-plan.md for full plan)"
  fi
  echo "=== END ACTIVE PLAN ==="
fi

exit 0
