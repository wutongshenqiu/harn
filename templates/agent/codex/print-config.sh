#!/bin/sh
set -eu

ROOT=$(CDPATH= cd -- "$(dirname -- "$0")/../.." && pwd)
SKILLS_DIR="$ROOT/.agents/skills"

if [ ! -d "$SKILLS_DIR" ]; then
  echo "No Codex skills found at $SKILLS_DIR" >&2
  exit 1
fi

find "$SKILLS_DIR" -mindepth 2 -maxdepth 2 -name 'SKILL.md' | sort | while read -r skill; do
  printf '[[skills.config]]\n'
  printf 'path = "%s"\n' "$skill"
  printf 'enabled = true\n\n'
done
