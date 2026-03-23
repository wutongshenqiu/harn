use harn_core::agent_tools::{AgentToolArtifact, agent_tool};
use harn_core::config::HarnConfig;
use harn_core::doctor::{CheckResult, Diagnostic, Severity};
use std::path::Path;

/// Check CI workflow files exist for configured workflows.
pub fn check_ci(root: &Path, config: &HarnConfig) -> CheckResult {
    let mut result = CheckResult::default();
    let check = "ci";

    let Some(ci) = &config.modules.ci else {
        CheckResult::ok(check, "CI module not configured, skipping");
        return result;
    };

    let provider = ci.provider.as_str();
    for workflow in &ci.workflows {
        let path = match provider {
            "github" => format!(".github/workflows/{workflow}.yml"),
            "gitlab" if workflow == "ci" => ".gitlab-ci.yml".to_string(),
            "gitlab" => format!(".gitlab/{workflow}.yml"),
            "gitea" | "codeberg" => format!(".gitea/workflows/{workflow}.yml"),
            _ => format!(".ci/{workflow}.yml"),
        };

        if root.join(&path).exists() {
            CheckResult::ok(check, format!("{path} exists"));
        } else {
            result.push(Diagnostic {
                severity: Severity::Warning,
                check: check.into(),
                message: format!("{path} missing (workflow: {workflow})"),
                fix: None,
            });
        }
    }

    result
}

/// Check agent config files exist for configured tools.
pub fn check_agent(root: &Path, config: &HarnConfig) -> CheckResult {
    let mut result = CheckResult::default();
    let check = "agent";

    let Some(agent) = &config.modules.agent else {
        CheckResult::ok(check, "Agent module not configured, skipping");
        return result;
    };

    let has_agents_md = root.join("AGENTS.md").exists();
    if has_agents_md {
        CheckResult::ok(check, "AGENTS.md exists");
    } else {
        result.push(Diagnostic {
            severity: Severity::Warning,
            check: check.into(),
            message: "AGENTS.md missing".into(),
            fix: None,
        });
    }

    let has_claude_md = root.join("CLAUDE.md").exists();
    if agent.tools.iter().any(|tool| tool == "claude") {
        if has_claude_md {
            CheckResult::ok(check, "CLAUDE.md exists");
        } else {
            result.push(Diagnostic {
                severity: Severity::Warning,
                check: check.into(),
                message: "CLAUDE.md missing (tool: claude)".into(),
                fix: None,
            });
        }
    } else if has_claude_md {
        CheckResult::ok(check, "CLAUDE.md exists (supplemental context)");
    }

    for tool_id in &agent.tools {
        let Some(tool) = agent_tool(tool_id) else {
            result.push(Diagnostic {
                severity: Severity::Error,
                check: check.into(),
                message: format!("unsupported agent tool configured: {tool_id}"),
                fix: None,
            });
            continue;
        };

        if let Some(note) = tool.doctor_note
            && has_agents_md
        {
            CheckResult::ok(check, format!("{note} ({tool_id})"));
        }

        for artifact in tool.artifacts {
            check_agent_artifact(
                root,
                *artifact,
                tool_id,
                &agent.commands,
                &mut result,
                check,
            );
        }
    }

    if agent.tools.iter().any(|tool| tool == "claude") {
        let settings_path = root.join(".claude/settings.json");
        if settings_path.exists()
            && let Ok(content) = std::fs::read_to_string(&settings_path)
        {
            check_package_manager_consistency(root, &content, &mut result, check);
        }
    }

    result
}

fn check_agent_artifact(
    root: &Path,
    artifact: AgentToolArtifact,
    tool_id: &str,
    commands: &[String],
    result: &mut CheckResult,
    check: &str,
) {
    for path in artifact.expected_paths(commands) {
        if root.join(&path).exists() {
            CheckResult::ok(check, format!("{path} exists ({tool_id})"));
        } else {
            result.push(Diagnostic {
                severity: Severity::Warning,
                check: check.into(),
                message: format!("{path} missing (tool: {tool_id})"),
                fix: None,
            });
        }
    }
}

fn check_package_manager_consistency(
    root: &Path,
    settings_content: &str,
    result: &mut CheckResult,
    check: &str,
) {
    // Detect actual package manager from lockfiles
    let actual = if root.join("bun.lockb").exists() || root.join("bun.lock").exists() {
        Some("bun")
    } else if root.join("pnpm-lock.yaml").exists() {
        Some("pnpm")
    } else if root.join("yarn.lock").exists() {
        Some("yarn")
    } else if root.join("package-lock.json").exists() {
        Some("npm")
    } else {
        None
    };

    if let Some(pkg) = actual {
        if !settings_content.contains(&format!("Bash({pkg}:*)")) {
            result.push(Diagnostic {
                severity: Severity::Warning,
                check: check.into(),
                message: format!(
                    ".claude/settings.json permissions don't include {pkg} (detected from lockfile)"
                ),
                fix: None,
            });
        }
    }
}

/// Check build files match configured build tool and validate required targets.
pub fn check_build(root: &Path, config: &HarnConfig) -> CheckResult {
    let mut result = CheckResult::default();
    let check = "build";

    let Some(build) = &config.modules.build else {
        CheckResult::ok(check, "Build module not configured, skipping");
        return result;
    };

    let expected_file = match build.tool.as_str() {
        "make" => "Makefile",
        "just" => "Justfile",
        "task" => "Taskfile.yml",
        _ => return result,
    };

    let file_path = root.join(expected_file);
    if file_path.exists() {
        CheckResult::ok(check, format!("{expected_file} exists"));

        // Validate required targets
        if let Ok(content) = std::fs::read_to_string(&file_path) {
            check_build_targets(
                &content,
                expected_file,
                build.tool.as_str(),
                &mut result,
                check,
            );
        }
    } else {
        result.push(Diagnostic {
            severity: Severity::Warning,
            check: check.into(),
            message: format!("{expected_file} missing (configured tool: {})", build.tool),
            fix: None,
        });
    }

    result
}

fn check_build_targets(
    content: &str,
    file_name: &str,
    tool: &str,
    result: &mut CheckResult,
    check: &str,
) {
    let required_targets = ["build", "test", "lint", "fmt"];

    for target in &required_targets {
        let has_target = match tool {
            "make" => {
                // Makefile: look for `target:` at start of line
                content
                    .lines()
                    .any(|line| line.starts_with(&format!("{target}:")))
            }
            "just" => {
                // Justfile: look for `target:` at start of line (same syntax)
                content
                    .lines()
                    .any(|line| line.starts_with(&format!("{target}:")))
            }
            "task" => {
                // Taskfile.yml: look for `target:` as a task key (indented)
                content.contains(&format!("  {target}:"))
                    || content.contains(&format!("\t{target}:"))
            }
            _ => true,
        };

        if has_target {
            CheckResult::ok(check, format!("{file_name} has '{target}' target"));
        } else {
            result.push(Diagnostic {
                severity: Severity::Warning,
                check: check.into(),
                message: format!("{file_name} missing '{target}' target"),
                fix: None,
            });
        }
    }
}

/// Check git config files and .gitignore coverage.
pub fn check_git(root: &Path, config: &HarnConfig) -> CheckResult {
    let mut result = CheckResult::default();
    let check = "git";

    let Some(git) = &config.modules.git else {
        CheckResult::ok(check, "Git module not configured, skipping");
        return result;
    };

    if git.gitignore {
        let gitignore_path = root.join(".gitignore");
        if gitignore_path.exists() {
            CheckResult::ok(check, ".gitignore exists");

            // Validate language-specific coverage
            if let Ok(content) = std::fs::read_to_string(&gitignore_path) {
                check_gitignore_coverage(&content, config, &mut result, check);
            }
        } else {
            result.push(Diagnostic {
                severity: Severity::Warning,
                check: check.into(),
                message: ".gitignore missing (gitignore = true)".into(),
                fix: None,
            });
        }
    }

    if git.gitattributes {
        if root.join(".gitattributes").exists() {
            CheckResult::ok(check, ".gitattributes exists");
        } else {
            result.push(Diagnostic {
                severity: Severity::Warning,
                check: check.into(),
                message: ".gitattributes missing (gitattributes = true)".into(),
                fix: None,
            });
        }
    }

    result
}

fn check_gitignore_coverage(
    content: &str,
    config: &HarnConfig,
    result: &mut CheckResult,
    check: &str,
) {
    let required: &[(&str, &[&str])] = &[
        ("rust", &["target/"]),
        ("go", &["bin/"]),
        ("typescript", &["node_modules/"]),
        ("javascript", &["node_modules/"]),
        ("python", &["__pycache__/", "*.pyc"]),
        ("java", &["*.class"]),
        ("dart", &[".dart_tool/"]),
        ("flutter", &[".dart_tool/"]),
        ("cpp", &["*.o"]),
        ("c", &["*.o"]),
    ];

    for lang in &config.stacks.languages {
        if let Some((_, patterns)) = required.iter().find(|(l, _)| *l == lang.as_str()) {
            for pattern in *patterns {
                if content.contains(pattern) {
                    CheckResult::ok(check, format!(".gitignore covers {lang} ({pattern})"));
                } else {
                    result.push(Diagnostic {
                        severity: Severity::Warning,
                        check: check.into(),
                        message: format!(".gitignore missing '{pattern}' for {lang}"),
                        fix: None,
                    });
                }
            }
        }
    }
}

/// Check quality config files and linter presence.
pub fn check_quality(root: &Path, config: &HarnConfig) -> CheckResult {
    let mut result = CheckResult::default();
    let check = "quality";

    let Some(quality) = &config.modules.quality else {
        CheckResult::ok(check, "Quality module not configured, skipping");
        return result;
    };

    if quality.editorconfig {
        if root.join(".editorconfig").exists() {
            CheckResult::ok(check, ".editorconfig exists");
        } else {
            result.push(Diagnostic {
                severity: Severity::Warning,
                check: check.into(),
                message: ".editorconfig missing (editorconfig = true)".into(),
                fix: None,
            });
        }
    }

    // Check language-specific linter configs
    for lang in &config.stacks.languages {
        check_linter_config(root, lang, &mut result, check);
    }

    result
}

fn check_linter_config(root: &Path, lang: &str, result: &mut CheckResult, check: &str) {
    let (config_name, paths): (&str, &[&str]) = match lang {
        "rust" => ("clippy config", &["clippy.toml", ".clippy.toml"]),
        "go" => ("golangci-lint config", &[".golangci.yml", ".golangci.yaml"]),
        "typescript" | "javascript" => (
            "eslint config",
            &[
                "eslint.config.js",
                "eslint.config.mjs",
                "eslint.config.cjs",
                ".eslintrc.json",
                ".eslintrc.js",
                ".eslintrc.yml",
            ],
        ),
        "python" => (
            "ruff/linter config",
            &["ruff.toml", ".ruff.toml", "pyproject.toml"],
        ),
        "java" => ("checkstyle config", &["checkstyle.xml"]),
        "cpp" | "c" => ("clang-format config", &[".clang-format", ".clang-tidy"]),
        _ => return,
    };

    let found = paths.iter().any(|p| root.join(p).exists());

    // For rust, also accept workspace Cargo.toml with clippy lints
    let found = if lang == "rust" && !found {
        root.join("Cargo.toml")
            .exists()
            .then(|| std::fs::read_to_string(root.join("Cargo.toml")).ok())
            .flatten()
            .is_some_and(|c| c.contains("[workspace.lints.clippy]") || c.contains("[lints.clippy]"))
    } else {
        found
    };

    if found {
        CheckResult::ok(check, format!("{config_name} found ({lang})"));
    } else {
        result.push(Diagnostic {
            severity: Severity::Warning,
            check: check.into(),
            message: format!("no {config_name} found for {lang}"),
            fix: None,
        });
    }
}

/// Run all project-wide checks (non-SDD).
pub fn run_all_project_checks(root: &Path, config: &HarnConfig) -> CheckResult {
    let mut result = CheckResult::default();

    println!("{}", console::style("[CI]").bold());
    result.merge(check_ci(root, config));
    println!();

    println!("{}", console::style("[Agent]").bold());
    result.merge(check_agent(root, config));
    println!();

    println!("{}", console::style("[Build]").bold());
    result.merge(check_build(root, config));
    println!();

    println!("{}", console::style("[Git]").bold());
    result.merge(check_git(root, config));
    println!();

    println!("{}", console::style("[Quality]").bold());
    result.merge(check_quality(root, config));

    result
}

#[cfg(test)]
mod tests {
    use super::check_agent;
    use harn_core::config::{AgentConfig, HarnConfig, ModulesConfig, ProjectConfig, StacksConfig};

    fn config_with_agent(tools: &[&str], commands: &[&str]) -> HarnConfig {
        HarnConfig {
            project: ProjectConfig {
                name: "agent-checks".into(),
                r#type: "single".into(),
            },
            stacks: StacksConfig::default(),
            modules: ModulesConfig {
                agent: Some(AgentConfig {
                    tools: tools.iter().map(ToString::to_string).collect(),
                    commands: commands.iter().map(ToString::to_string).collect(),
                    ..AgentConfig::default()
                }),
                ..ModulesConfig::default()
            },
        }
    }

    #[test]
    fn codex_passes_with_agents_md_only() {
        let dir = tempfile::tempdir().expect("tempdir should be created");
        std::fs::write(dir.path().join("AGENTS.md"), "# Agent Context\n")
            .expect("AGENTS.md should be written");

        let result = check_agent(dir.path(), &config_with_agent(&["codex"], &[]));

        assert_eq!(result.warning_count(), 0);
        assert_eq!(result.error_count(), 0);
    }

    #[test]
    fn opencode_checks_command_files() {
        let dir = tempfile::tempdir().expect("tempdir should be created");
        std::fs::write(dir.path().join("AGENTS.md"), "# Agent Context\n")
            .expect("AGENTS.md should be written");

        let result = check_agent(dir.path(), &config_with_agent(&["opencode"], &["review"]));

        assert_eq!(result.warning_count(), 1);
        assert!(
            result
                .diagnostics
                .iter()
                .any(|diag| diag.message == ".opencode/commands/review.md missing (tool: opencode)")
        );
    }

    #[test]
    fn unknown_tool_is_reported_as_error() {
        let dir = tempfile::tempdir().expect("tempdir should be created");
        std::fs::write(dir.path().join("AGENTS.md"), "# Agent Context\n")
            .expect("AGENTS.md should be written");

        let result = check_agent(dir.path(), &config_with_agent(&["unknown"], &[]));

        assert_eq!(result.error_count(), 1);
        assert!(
            result
                .diagnostics
                .iter()
                .any(|diag| diag.message == "unsupported agent tool configured: unknown")
        );
    }
}
