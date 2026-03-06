use anyhow::Result;
use console::style;
use dialoguer::{Confirm, FuzzySelect, Input, MultiSelect};
use harn_core::config::*;
use std::path::Path;

/// Interactive configuration gathering.
///
/// Guides the user through module selection with sensible defaults.
pub fn gather_config(root: &Path) -> Result<HarnConfig> {
    let default_name = root
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("project")
        .to_string();

    // Project basics
    let name: String = Input::new()
        .with_prompt("Project name")
        .default(default_name)
        .interact_text()?;

    let type_options = &["single", "monorepo"];
    let type_idx = FuzzySelect::new()
        .with_prompt("Project type")
        .items(type_options)
        .default(0)
        .interact()?;
    let project_type = type_options[type_idx].to_string();

    // Languages
    let lang_options = &["rust", "go", "typescript", "python", "dart", "java", "none"];
    let lang_defaults = detect_languages(root);
    let lang_indices = MultiSelect::new()
        .with_prompt("Languages (space to select, enter to confirm)")
        .items(lang_options)
        .defaults(&lang_defaults)
        .interact()?;
    let languages: Vec<String> = lang_indices
        .iter()
        .map(|&i| lang_options[i].to_string())
        .filter(|l| l != "none")
        .collect();

    // Frameworks (based on selected languages)
    let frameworks = gather_frameworks(&languages)?;

    // Modules
    println!();
    println!("{}", style("Select modules to include:").bold());
    let module_options = &[
        ("sdd", "SDD documentation (specs, reference, playbooks)"),
        ("ci", "CI/CD pipelines"),
        ("agent", "AI agent configs (Claude, Cursor, etc.)"),
        ("build", "Build orchestration (Make, Just, Task)"),
        ("ide", "IDE/editor configs"),
        ("git", "Git config (.gitignore, hooks)"),
        ("docker", "Docker (Dockerfile, Compose)"),
        ("env", "Environment (.env.example)"),
        ("quality", "Code quality (EditorConfig, linters)"),
    ];

    let module_labels: Vec<String> = module_options
        .iter()
        .map(|(id, desc)| format!("{id:<10} {desc}"))
        .collect();
    let module_defaults = vec![true; module_options.len()]; // All selected by default

    let selected_modules = MultiSelect::new()
        .with_prompt("Modules (space to toggle)")
        .items(&module_labels)
        .defaults(&module_defaults)
        .interact()?;

    let enabled: Vec<&str> = selected_modules
        .iter()
        .map(|&i| module_options[i].0)
        .collect();

    // Module-specific config
    let ci = if enabled.contains(&"ci") {
        Some(gather_ci_config()?)
    } else {
        None
    };

    let agent = if enabled.contains(&"agent") {
        Some(gather_agent_config()?)
    } else {
        None
    };

    let build = if enabled.contains(&"build") {
        Some(gather_build_config()?)
    } else {
        None
    };

    let ide = if enabled.contains(&"ide") {
        Some(gather_ide_config()?)
    } else {
        None
    };

    Ok(HarnConfig {
        project: ProjectConfig {
            name,
            r#type: project_type,
        },
        stacks: StacksConfig {
            languages,
            frameworks,
        },
        modules: ModulesConfig {
            sdd: if enabled.contains(&"sdd") {
                Some(SddConfig::default())
            } else {
                None
            },
            ci,
            agent,
            build,
            ide,
            git: if enabled.contains(&"git") {
                Some(GitConfig::default())
            } else {
                None
            },
            docker: if enabled.contains(&"docker") {
                Some(DockerConfig::default())
            } else {
                None
            },
            env: if enabled.contains(&"env") {
                Some(EnvConfig::default())
            } else {
                None
            },
            quality: if enabled.contains(&"quality") {
                Some(QualityConfig::default())
            } else {
                None
            },
        },
    })
}

fn gather_ci_config() -> Result<CiConfig> {
    let providers = &["github", "gitlab", "gitea"];
    let idx = FuzzySelect::new()
        .with_prompt("CI/CD provider")
        .items(providers)
        .default(0)
        .interact()?;

    let workflow_options = &["ci", "cd", "security"];
    let workflow_defaults = vec![true, true, false];
    let selected = MultiSelect::new()
        .with_prompt("Workflows")
        .items(workflow_options)
        .defaults(&workflow_defaults)
        .interact()?;

    Ok(CiConfig {
        provider: providers[idx].to_string(),
        workflows: selected
            .iter()
            .map(|&i| workflow_options[i].to_string())
            .collect(),
    })
}

fn gather_agent_config() -> Result<AgentConfig> {
    let tool_options = &["claude", "cursor", "windsurf", "cline", "opencode"];
    let tool_defaults = vec![true, false, false, false, false];
    let selected = MultiSelect::new()
        .with_prompt("AI coding tools")
        .items(tool_options)
        .defaults(&tool_defaults)
        .interact()?;

    let tools: Vec<String> = selected
        .iter()
        .map(|&i| tool_options[i].to_string())
        .collect();

    let pre_commit = Confirm::new()
        .with_prompt("Enable pre-commit hook (lint + test)?")
        .default(true)
        .interact()?;

    Ok(AgentConfig {
        tools,
        pre_commit_hook: pre_commit,
        ..Default::default()
    })
}

fn gather_build_config() -> Result<BuildConfig> {
    let tools = &["make", "just", "task"];
    let idx = FuzzySelect::new()
        .with_prompt("Build tool")
        .items(tools)
        .default(0)
        .interact()?;

    Ok(BuildConfig {
        tool: tools[idx].to_string(),
    })
}

fn gather_ide_config() -> Result<IdeConfig> {
    let editors = &["vscode", "jetbrains", "zed", "vim"];
    let defaults = vec![true, false, false, false];
    let selected = MultiSelect::new()
        .with_prompt("Editors/IDEs")
        .items(editors)
        .defaults(&defaults)
        .interact()?;

    Ok(IdeConfig {
        editors: selected.iter().map(|&i| editors[i].to_string()).collect(),
    })
}

fn gather_frameworks(languages: &[String]) -> Result<Vec<String>> {
    let mut frameworks = Vec::new();

    for lang in languages {
        let options: &[&str] = match lang.as_str() {
            "rust" => &["axum", "actix-web", "rocket", "none"],
            "go" => &["chi", "gin", "echo", "fiber", "none"],
            "typescript" => &["react", "nextjs", "vue", "svelte", "express", "none"],
            "python" => &["fastapi", "django", "flask", "none"],
            "dart" => &["flutter", "none"],
            _ => continue,
        };

        if options.len() > 1 {
            let idx = FuzzySelect::new()
                .with_prompt(format!("Framework for {lang}"))
                .items(options)
                .default(0)
                .interact()?;
            if options[idx] != "none" {
                frameworks.push(options[idx].to_string());
            }
        }
    }

    Ok(frameworks)
}

fn detect_languages(root: &Path) -> Vec<bool> {
    // [rust, go, typescript, python, dart, java, none]
    vec![
        root.join("Cargo.toml").exists(),
        root.join("go.mod").exists(),
        root.join("package.json").exists() || root.join("tsconfig.json").exists(),
        root.join("pyproject.toml").exists() || root.join("setup.py").exists(),
        root.join("pubspec.yaml").exists(),
        root.join("pom.xml").exists() || root.join("build.gradle").exists(),
        false, // none
    ]
}
