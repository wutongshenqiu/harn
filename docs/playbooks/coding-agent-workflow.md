# Coding Agent Workflow

Core principle: **Read docs first, write code, update docs last.**

## Flow

```
1. Read spec + reference docs
2. Implement (data -> logic -> API -> UI -> tests)
3. Verify (make lint && make test)
4. Evaluate results
   - Code-level issue → fix and go to step 3
   - Design-level issue → update spec, go to step 1
   - All good → continue
5. Update reference docs
6. Ship (/ship)
```

## When to Update Spec vs Fix in Code

**Update the spec** when:
- API shape or interface contract changes
- A task needs to be added, removed, or restructured
- Constraints or non-goals turn out to be wrong
- Architectural assumptions are invalidated

**Fix in code** when:
- Bug in implementation logic
- Test edge case not anticipated
- Performance tuning
- Linter / formatting issues

## Quality Checklist

- [ ] `make lint` passes
- [ ] `make test` passes
- [ ] New endpoints in `reference/api-conventions.md`
- [ ] New types in `reference/types/`
- [ ] DB changes have migrations
- [ ] User text uses i18n
- [ ] No hardcoded secrets
- [ ] Conventional commit message
