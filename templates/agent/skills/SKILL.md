---
name: {{ workflow_id }}
description: "{{ workflow_description }}"
---

# {{ workflow_title }}

Canonical command: `{{ workflow_command }}`

Purpose: {{ workflow_purpose }}

Use `AGENTS.md` as the repo-wide brief and treat `.agents/workflows/` as the neutral workflow
source when checking for drift.

{{ workflow_body }}
