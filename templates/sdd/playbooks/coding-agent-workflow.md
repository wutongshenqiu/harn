# Coding Agent Workflow

Core principle: **Read docs first, write code, update docs last.**

## Flow

```
1. Read spec + reference docs
2. Implement (data -> logic -> API -> UI -> tests)
3. Verify (make lint && make test)
4. Update reference docs
5. Ship (/ship)
```

## Quality Checklist

- [ ] `make lint` passes
- [ ] `make test` passes
- [ ] New endpoints in `reference/api-conventions.md`
- [ ] New types in `reference/types/`
- [ ] DB changes have migrations
- [ ] User text uses i18n
- [ ] No hardcoded secrets
- [ ] Conventional commit message
