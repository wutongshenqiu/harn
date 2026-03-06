# harn

Universal project harness with Spec-Driven Development (SDD) methodology.

[![CI](https://github.com/nicepkg/harn/actions/workflows/ci.yml/badge.svg)](https://github.com/nicepkg/harn/actions/workflows/ci.yml)
[![Release](https://github.com/nicepkg/harn/actions/workflows/release.yml/badge.svg)](https://github.com/nicepkg/harn/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

**harn** equips your project with production-grade development infrastructure in one command: SDD documentation, AI agent configs, CI/CD pipelines, build orchestration, and quality gates.

Distilled from patterns across multiple production Rust/Go/TypeScript/Flutter projects.

## Install

### From GitHub Releases

```bash
# macOS (Apple Silicon)
curl -sSL https://github.com/nicepkg/harn/releases/latest/download/harn-aarch64-apple-darwin.tar.gz | tar xz
sudo mv harn /usr/local/bin/

# macOS (Intel)
curl -sSL https://github.com/nicepkg/harn/releases/latest/download/harn-x86_64-apple-darwin.tar.gz | tar xz
sudo mv harn /usr/local/bin/

# Linux (x86_64)
curl -sSL https://github.com/nicepkg/harn/releases/latest/download/harn-x86_64-unknown-linux-gnu.tar.gz | tar xz
sudo mv harn /usr/local/bin/
```

### From Source

```bash
cargo install --git https://github.com/nicepkg/harn
```

## Quick Start

```bash
# Interactive setup — select languages, modules, tools
harn init my-project

# Config-driven setup — reproducible across teams
harn init my-project --config harn.toml

# Add a single module to an existing project
harn add sdd
harn add agent
harn add ci
```

## What It Generates

```
project/
├── CLAUDE.md                     # AI agent context
├── AGENTS.md                     # Universal agent context
├── Makefile                      # Unified build (language-aware)
├── harn.toml                     # Reproducible config
├── .claude/
│   ├── settings.json             # Permissions + pre-commit hook
│   └── commands/                 # 15 slash commands
├── .cursor/rules                 # Cursor AI rules
├── .windsurfrules                # Windsurf AI rules
├── .github/workflows/            # CI/CD pipelines
├── .vscode/                      # Editor settings
├── .editorconfig                 # Cross-editor formatting
└── docs/
    ├── specs/                    # Spec-Driven Development
    │   ├── _index.md             # Spec registry
    │   ├── _templates/           # PRD, TD, Research templates
    │   ├── active/               # In-progress specs
    │   └── completed/            # Shipped specs
    ├── reference/                # SSOT documentation
    │   ├── types/                # Enums, models, DTOs
    │   ├── api-conventions.md    # API standards
    │   └── data-model.md         # Database schema
    └── playbooks/                # How-to guides
```

## Modules

| Module | Description | Key Options |
|--------|-------------|-------------|
| `sdd` | Spec-Driven Development docs | playbooks, reference |
| `ci` | CI/CD pipelines | github, gitlab, gitea |
| `agent` | AI coding agent configs | claude, cursor, windsurf, cline, opencode |
| `build` | Build orchestration | make, just, task |
| `ide` | Editor configuration | vscode, zed, jetbrains |
| `git` | Git config | .gitignore (language-aware) |
| `docker` | Containerization | Dockerfile, Compose |
| `env` | Environment management | .env.example |
| `quality` | Code quality tooling | EditorConfig, linters |

## Configuration

`harn.toml` makes your setup reproducible and shareable:

```toml
[project]
name = "my-api"
type = "single"  # or "monorepo"

[stacks]
languages = ["rust", "typescript"]
frameworks = ["axum", "react"]

[modules.sdd]
playbooks = true
reference = true

[modules.ci]
provider = "github"       # github | gitlab | gitea
workflows = ["ci", "cd"]

[modules.agent]
tools = ["claude", "cursor"]
commands = ["ship", "implement", "spec", "lint", "test", "review"]
pre_commit_hook = true

[modules.build]
tool = "make"             # make | just | task

[modules.ide]
editors = ["vscode"]

[modules.git]
gitignore = true

[modules.quality]
editorconfig = true
```

Generate a full example: `harn example`

## Language Support

harn generates language-aware Makefiles, .gitignore, linter configs, and CI workflows:

| Language | Makefile | .gitignore | Linter Config | CI |
|----------|----------|------------|---------------|-----|
| Rust | cargo targets | target/ | rust-toolchain.toml | clippy + fmt |
| Go | go targets | bin/ | .golangci.yml | golangci-lint |
| TypeScript | npm/pnpm targets | node_modules/ | eslint + prettier | lint + tsc |
| Dart/Flutter | flutter targets | .dart_tool/ | analysis_options | analyze |
| Python | pip/uv targets | __pycache__/ | — | — |

## Methodology

harn implements **Harness Engineering** — treating developer infrastructure as a product:

1. **Convention over Configuration** — sensible defaults, escape hatches via `harn.toml`
2. **Spec-Driven Development** — define features as Specs (PRD + TD), track lifecycle
3. **AI-Agent-First** — CLAUDE.md + slash commands for AI-assisted workflows
4. **Quality Gates** — pre-commit hooks enforce `make lint && make test`
5. **SSOT Documentation** — reference docs as single source of truth
6. **Reproducible** — `harn.toml` captures all decisions for team sharing

## Commands

```
harn init [dir]              Initialize a new project (interactive)
harn init [dir] -c harn.toml Config-driven init (non-interactive)
harn add <module> [dir]      Add a module to existing project
harn spec [title] -d [dir]   Create a new Spec
harn modules                 List available modules
harn example                 Generate example harn.toml
```

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

MIT
