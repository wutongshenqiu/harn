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
    ];

    for cmd in &default_commands {
        let path = format!("agent/commands/{cmd}.md");
        assert!(
            engine.has_template(&path),
            "Missing command template: {path}"
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
