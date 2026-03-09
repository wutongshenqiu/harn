use harn_modules::sdd::SDD_PLAYBOOK_FILES;
use harn_templates::TemplateEngine;

/// Verify that all build tool + language combinations have templates.
#[test]
fn build_templates_exist_for_all_tools() {
    let engine = TemplateEngine::new();
    let tools = ["make", "just", "task"];
    let languages = ["rust", "go", "typescript", "dart", "generic"];

    for tool in &tools {
        for lang in &languages {
            let path = format!("build/{tool}/{lang}");
            assert!(engine.has_template(&path), "Missing build template: {path}");
        }
    }
}

/// Verify that all IDE editors with code paths have templates.
#[test]
fn ide_templates_exist_for_supported_editors() {
    let engine = TemplateEngine::new();

    // Editors that have match arms in ide.rs
    let editors_with_code = [
        (
            "vscode",
            vec!["ide/vscode/settings.json", "ide/vscode/extensions.json"],
        ),
        ("zed", vec!["ide/zed/settings.json"]),
    ];

    for (editor, templates) in &editors_with_code {
        for path in templates {
            assert!(
                engine.has_template(path),
                "Missing IDE template for {editor}: {path}"
            );
        }
    }
}

/// Verify that all default slash commands have template files.
#[test]
fn command_templates_exist_for_all_defaults() {
    let engine = TemplateEngine::new();
    let default_commands = [
        "ship",
        "implement",
        "spec",
        "lint",
        "test",
        "review",
        "diagnose",
        "deps",
        "issues",
        "doc-audit",
        "retro",
        "sync-commands",
        "ci",
        "pr",
        "deploy",
        "run-plan",
    ];

    for cmd in &default_commands {
        let path = format!("agent/commands/{cmd}.md");
        assert!(
            engine.has_template(&path),
            "Missing command template: {path}"
        );
    }
}

/// Verify that every command template has a description entry in agent.rs.
/// Reads the source file and checks that each template name appears in
/// `build_slash_commands_table`.
#[test]
fn slash_command_descriptions_cover_all_templates() {
    let engine = TemplateEngine::new();
    let templates = engine.list_templates("agent/commands/");

    let agent_src =
        std::fs::read_to_string("src/agent.rs").expect("Could not read agent.rs source");

    for tpl in &templates {
        // template path is "agent/commands/foo.md", extract "foo"
        let name = tpl
            .strip_prefix("agent/commands/")
            .and_then(|s| s.strip_suffix(".md"))
            .unwrap_or(tpl);
        assert!(
            agent_src.contains(&format!("\"{name}\"")),
            "Command template '{name}' exists but has no entry in build_slash_commands_table() — add it to agent.rs"
        );
    }
}

/// Verify that every playbook in templates/sdd/playbooks/ is listed in `SDD_PLAYBOOK_FILES`.
#[test]
fn playbook_templates_covered_by_sdd_module() {
    let engine = TemplateEngine::new();
    let embedded = engine.list_templates("sdd/playbooks/");

    // Every embedded playbook must be in the const
    for path in &embedded {
        assert!(
            SDD_PLAYBOOK_FILES.contains(&path.as_str()),
            "Playbook template '{path}' exists in templates/ but is not listed in SDD_PLAYBOOK_FILES — add it to sdd.rs"
        );
    }

    // Every entry in the const must exist as an embedded template
    for path in SDD_PLAYBOOK_FILES {
        assert!(
            engine.has_template(path),
            "SDD_PLAYBOOK_FILES lists '{path}' but the template file does not exist"
        );
    }
}

/// Verify that all CI providers have at least a ci.yml template.
#[test]
fn ci_templates_exist_for_all_providers() {
    let engine = TemplateEngine::new();
    let providers = ["github", "gitlab", "gitea"];

    for provider in &providers {
        let path = format!("ci/{provider}/ci.yml");
        assert!(
            engine.has_template(&path),
            "Missing CI template for {provider}: {path}"
        );
    }
}
