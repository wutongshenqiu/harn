# harn

Universal project harness with SDD methodology. Rust workspace CLI tool.

## Architecture

```
crates/
  cli/        # Binary: clap CLI + dialoguer interactive prompts
  core/       # Library: config types, Module trait, ProjectContext
  modules/    # Library: 9 built-in modules (sdd, ci, agent, build, ...)
  templates/  # Library: minijinja engine + include_dir compile-time embedding

templates/    # Template files embedded into the binary at compile time
```

## Commands

```bash
make check        # fmt-check + clippy + test (full CI locally)
make lint         # cargo fmt --check + clippy
make test         # cargo test --workspace
make fmt          # cargo fmt
make build        # cargo build
make release      # cargo build --release
```

## Slash Commands

| Command | Purpose |
|---------|---------|
| `/ship [msg]` | Lint + test + commit + push + PR |
| `/implement SPEC-NNN` | Implement a spec |
| `/spec create/list/advance` | Manage spec lifecycle |
| `/lint [fix]` | Run linters |
| `/test [scope]` | Run tests |
| `/review [PR#]` | Code review |
| `/diagnose [error]` | Diagnose issues |
| `/deps [check/update]` | Manage dependencies |
| `/doc-audit` | Audit docs vs code |
| `/issues SPEC-NNN` | Generate issues from Spec |
| `/retro` | Session retrospective |
| `/ci [PR#]` | Check CI status |
| `/pr [title]` | Create pull request |
| `/deploy` | Deploy |
| `/sync-commands` | Sync slash commands |

## Extension Points

- New module: implement Module trait in `crates/modules/src/`, register in `registry.rs`, add templates
- New CI provider: add templates in `templates/ci/<provider>/`, update `crates/modules/src/ci.rs`
- New language: add Makefile template, .gitignore fragment, linter config, CI steps
- New AI tool: add template in `templates/agent/`, update `crates/modules/src/agent.rs`

## Coding Rules

1. **Lint before commit** — `make lint` must pass (clippy pedantic, zero warnings)
2. **Test before commit** — `make test` must pass
3. **No unsafe** — `unsafe_code = "forbid"`
4. **Conventional commits** — `feat:`, `fix:`, `docs:`, `refactor:`, `test:`, `chore:`
5. **Edition 2024**, rust-version 1.87, resolver 3
6. **All deps in workspace Cargo.toml**
7. **Templates use minijinja syntax** — `{{ variable }}`

## SDD (Spec-Driven Development)

```
Draft -> Active -> Completed
```

- Specs: `docs/specs/` (registry: `_index.md`)
- Playbooks: `docs/playbooks/`
