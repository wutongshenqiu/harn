Implement from Spec. Argument $ARGUMENTS: `SPEC-NNN [task N]`

Steps:

1. **Read Spec**: Read `docs/specs/active/$ARGUMENTS/` — TD first, PRD fallback
2. **Create branch**: `git checkout -b feature/$ARGUMENTS`
3. **Analyze dependencies**: Parse task ordering from TD
4. **Generate plan**: TaskList from TD's Task Breakdown
5. **Implement**: Execute tasks in dependency order
   - Build check after each task
   - Format after each task
6. **Quality verification**: `make fmt && make lint && make test`
7. **Report**: Completed tasks, modified files, next step: `/ship`