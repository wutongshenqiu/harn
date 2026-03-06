Generate issues from Spec. Argument $ARGUMENTS: `SPEC-NNN`

Steps:
1. Read `docs/specs/active/$ARGUMENTS/technical-design.md`
2. Check existing issues: `gh issue list --search "SPEC-NNN"`
3. Create Epic issue with task checklist
4. Create sub-task issues with labels
5. Report created issues with URLs