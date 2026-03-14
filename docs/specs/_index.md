# Spec Registry

All specifications for harn.

## Completed

| ID       | Title                                          | Status    | Location                        |
|----------|------------------------------------------------|-----------|---------------------------------|
| SPEC-001 | Complete Python Language Support                | Completed | completed/SPEC-001/             |
| SPEC-002 | Complete TypeScript Quality Templates           | Completed | completed/SPEC-002/             |
| SPEC-003 | Add Java Language Support                       | Completed | completed/SPEC-003/             |
| SPEC-004 | Add C/C++ Language Support                      | Completed | completed/SPEC-004/             |
| SPEC-005 | Add Docker Templates for All Languages          | Completed | completed/SPEC-005/             |
| SPEC-006 | SDD Project Diagnostics and Upgrade (harn doctor) | Completed | completed/SPEC-006/             |

## Active

| ID       | Title                                          | Status    | Location                        |
|----------|------------------------------------------------|-----------|---------------------------------|
| SPEC-007 | Fix write pipeline, non-interactive issue, and doctor depth | Completed | [completed/SPEC-007/](completed/SPEC-007/) |

## How to Create a New Spec

1. Copy template from `_templates/`
2. Assign next SPEC-NNN ID
3. Place in `active/SPEC-NNN/`
4. Register in this table under **Active**
5. When complete, move to `completed/` and update status

See [playbooks/create-new-spec.md](../playbooks/create-new-spec.md).