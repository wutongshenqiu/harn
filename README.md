# harn

Universal project harness with Spec-Driven Development (SDD) methodology.

[![CI](https://github.com/wutongshenqiu/harn/actions/workflows/ci.yml/badge.svg)](https://github.com/wutongshenqiu/harn/actions/workflows/ci.yml)
[![Release](https://github.com/wutongshenqiu/harn/actions/workflows/release.yml/badge.svg)](https://github.com/wutongshenqiu/harn/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

**harn** equips your project with production-grade development infrastructure in one command: SDD documentation, AI agent configs, CI/CD pipelines, build orchestration, and quality gates.

Distilled from patterns across multiple production Rust/Go/TypeScript/Flutter/Python/Java/C++ projects.

## Install

### From GitHub Releases

```bash
# macOS (Apple Silicon)
curl -sSL https://github.com/wutongshenqiu/harn/releases/latest/download/harn-aarch64-apple-darwin.tar.gz | tar xz
sudo mv harn /usr/local/bin/

# macOS (Intel)
curl -sSL https://github.com/wutongshenqiu/harn/releases/latest/download/harn-x86_64-apple-darwin.tar.gz | tar xz
sudo mv harn /usr/local/bin/

# Linux (x86_64)
curl -sSL https://github.com/wutongshenqiu/harn/releases/latest/download/harn-x86_64-unknown-linux-gnu.tar.gz | tar xz
sudo mv harn /usr/local/bin/
```

### From Source

```bash
cargo install --git https://github.com/wutongshenqiu/harn
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
├── .claude/                      # Claude Code config + slash commands
├── .cursor/rules                 # Cursor rules
├── .windsurfrules                # Windsurf rules
├── .clinerules                   # Cline rules
├── .qoder/rules/                 # Qoder rules
├── .opencode/commands/           # OpenCode commands
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
| `ci` | CI/CD pipelines | github, gitlab, gitea/codeberg |
| `agent` | AI coding agent configs | claude, cursor, windsurf, cline, opencode, qoder |
| `build` | Build orchestration | make, just, task |
| `ide` | Editor configuration | vscode, zed (jetbrains, vim planned) |
| `git` | Git config | .gitignore (language-aware), .gitattributes |
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
gitattributes = true

[modules.env]
extra_vars = ["API_KEY", "STRIPE_SECRET"]

[modules.quality]
editorconfig = true
```

Generate a full example: `harn example`

## Language Support

harn generates language-aware Makefiles, .gitignore, linter configs, Docker templates, and CI workflows:

| Language | Build | .gitignore | Linter Config | Docker |
|----------|-------|------------|---------------|--------|
| Rust | cargo targets | target/ | rust-toolchain.toml | multi-stage (rust:slim) |
| Go | go targets | bin/ | .golangci.yml | multi-stage (golang:alpine) |
| TypeScript | npm targets | node_modules/ | eslint + prettier | multi-stage (node:alpine) |
| Dart/Flutter | flutter targets | .dart_tool/ | — | multi-stage (dart:stable) |
| Python | uv + ruff + pytest | \_\_pycache\_\_/ | ruff.toml | python:slim + uv |
| Java | gradle targets | build/, .gradle/ | checkstyle.xml | multi-stage (temurin:21) |
| C/C++ | cmake targets | build/, *.o | .clang-format | multi-stage (gcc + debian) |

## Methodology

harn implements **Harness Engineering** — treating developer infrastructure as a product:

1. **Convention over Configuration** — sensible defaults, escape hatches via `harn.toml`
2. **Spec-Driven Development** — define features as Specs (PRD + TD), track lifecycle
3. **AI-Agent-First** — CLAUDE.md + slash commands for AI-assisted workflows
4. **Quality Gates** — optional pre-commit hooks enforce `make lint && make test` (`pre_commit_hook = true`)
5. **SSOT Documentation** — reference docs as single source of truth
6. **Reproducible** — `harn.toml` captures all decisions for team sharing

## Commands

```
harn init [dir]              Initialize a new project (interactive)
harn init [dir] -c harn.toml Config-driven init (non-interactive)
harn init [dir] --dry-run    Preview (shows CREATE/FORCE/SKIP per file)
harn init [dir] --force      Overwrite existing files (backs up to .harn-backup/)
harn add <module> [dir]      Add a module to existing project
harn add <module> --dry-run  Preview module output
harn add <module> --force    Overwrite (backs up to .harn-backup/)
harn spec [title] -d [dir]   Create a new Spec
harn doctor [dir]            Diagnose project health (SDD + all modules)
harn doctor [dir] --fix      Auto-fix safe issues
harn modules                 List available modules
harn example                 Generate example harn.toml
harn issue                   Submit an issue (interactive)
harn issue --type bug --title "..." --body "..."
                             Non-interactive issue creation
echo "desc" | harn issue --type bug --title "..."
                             Read body from stdin
harn issue --open            Open browser to new issue page
```

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

MIT
