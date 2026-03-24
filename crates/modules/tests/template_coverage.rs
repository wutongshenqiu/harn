use harn_core::agent_tools::AGENT_TOOL_IDS;
use harn_core::agent_workflows::{AGENT_WORKFLOW_IDS, DEFAULT_AGENT_WORKFLOW_IDS};
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

/// Verify that all supported workflows have neutral workflow templates.
#[test]
fn workflow_templates_exist_for_all_supported_workflows() {
    let engine = TemplateEngine::new();

    for workflow in AGENT_WORKFLOW_IDS {
        let path = format!("agent/workflows/{workflow}.md");
        assert!(
            engine.has_template(&path),
            "Missing workflow template: {path}"
        );
    }
}

/// Verify that all default workflows are part of the supported workflow registry.
#[test]
fn default_workflows_are_supported() {
    for workflow in DEFAULT_AGENT_WORKFLOW_IDS {
        assert!(
            AGENT_WORKFLOW_IDS.contains(workflow),
            "Default workflow '{workflow}' must be supported"
        );
    }
}

/// Verify that Codex helper templates exist.
#[test]
fn codex_templates_exist() {
    let engine = TemplateEngine::new();
    assert!(engine.has_template("agent/skills/SKILL.md"));
    assert!(engine.has_template("agent/codex/print-config.sh"));
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

/// Verify that README mentions every supported agent tool value.
#[test]
fn readme_mentions_supported_agent_tools() {
    let readme =
        std::fs::read_to_string("../../README.md").expect("Could not read repository README");

    for tool in AGENT_TOOL_IDS {
        assert!(
            readme.contains(tool),
            "README.md should mention supported agent tool '{tool}'"
        );
    }
}
