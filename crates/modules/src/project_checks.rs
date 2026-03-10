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

    // Check CLAUDE.md exists
    if root.join("CLAUDE.md").exists() {
        CheckResult::ok(check, "CLAUDE.md exists");
    } else {
        result.push(Diagnostic {
            severity: Severity::Warning,
            check: check.into(),
            message: "CLAUDE.md missing".into(),
            fix: None,
        });
    }

    // Check AGENTS.md exists
    if root.join("AGENTS.md").exists() {
        CheckResult::ok(check, "AGENTS.md exists");
    } else {
        result.push(Diagnostic {
            severity: Severity::Warning,
            check: check.into(),
            message: "AGENTS.md missing".into(),
            fix: None,
        });
    }

    // Check tool-specific config files
    for tool in &agent.tools {
        let path = match tool.as_str() {
            "claude" => ".claude/settings.json",
            "cursor" => ".cursor/rules",
            "windsurf" => ".windsurfrules",
            "cline" => ".clinerules",
            "qoder" => ".qoder/rules/harn.md",
            _ => continue,
        };

        if root.join(path).exists() {
            CheckResult::ok(check, format!("{path} exists ({tool})"));
        } else {
            result.push(Diagnostic {
                severity: Severity::Warning,
                check: check.into(),
                message: format!("{path} missing (tool: {tool})"),
                fix: None,
            });
        }
    }

    // Check package manager consistency
    if agent.tools.iter().any(|t| t == "claude") {
        let settings_path = root.join(".claude/settings.json");
        if settings_path.exists() {
            if let Ok(content) = std::fs::read_to_string(&settings_path) {
                check_package_manager_consistency(root, &content, &mut result, check);
            }
        }
    }

    result
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

/// Check build files match configured build tool.
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

    if root.join(expected_file).exists() {
        CheckResult::ok(check, format!("{expected_file} exists"));
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

/// Check git config files.
pub fn check_git(root: &Path, config: &HarnConfig) -> CheckResult {
    let mut result = CheckResult::default();
    let check = "git";

    let Some(git) = &config.modules.git else {
        CheckResult::ok(check, "Git module not configured, skipping");
        return result;
    };

    if git.gitignore {
        if root.join(".gitignore").exists() {
            CheckResult::ok(check, ".gitignore exists");
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

/// Check quality config files.
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

    result
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
