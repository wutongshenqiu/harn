use anyhow::{Context, Result};
use harn_core::agent_workflows::{agent_workflow, validate_agent_workflows};
use harn_core::config::AgentConfig;
use harn_core::{HarnConfig, ProjectContext};
use harn_templates::TemplateEngine;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, HashMap};
use std::path::Path;

pub const AGENT_OVERLAY_MANIFEST_PATH: &str = ".harn/agent-overlays.json";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManagedOverlay {
    pub path: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AgentOverlayManifest {
    pub version: u32,
    pub artifacts: Vec<String>,
}

impl AgentOverlayManifest {
    #[must_use]
    pub fn new(artifacts: Vec<String>) -> Self {
        let mut artifacts = artifacts;
        artifacts.sort();
        artifacts.dedup();
        Self {
            version: 1,
            artifacts,
        }
    }

    #[must_use]
    pub fn artifact_set(&self) -> BTreeSet<String> {
        self.artifacts.iter().cloned().collect()
    }
}

pub fn overlay_template_vars(ctx: &ProjectContext) -> StringMap {
    let mut vars = TemplateEngine::vars_from_context(ctx);
    let agent_config = ctx.config.modules.agent.clone().unwrap_or_default();
    vars.insert(
        "slash_commands_table".into(),
        build_slash_commands_table(&agent_config.commands),
    );
    vars
}

pub fn collect_managed_overlays(ctx: &ProjectContext) -> Result<Vec<ManagedOverlay>> {
    let agent_config = ctx.config.modules.agent.clone().unwrap_or_default();
    validate_agent_workflows(&agent_config.commands)?;

    let engine = TemplateEngine::from_context(ctx);
    let vars = overlay_template_vars(ctx);
    let mut overlays = Vec::new();

    overlays.push(ManagedOverlay {
        path: "AGENTS.md".into(),
        content: engine.render("agent/AGENTS.md", &vars)?,
    });
    overlays.push(ManagedOverlay {
        path: "CLAUDE.md".into(),
        content: engine.render("agent/CLAUDE.md", &vars)?,
    });

    overlays.extend(collect_rule_overlays(&engine, &vars, &agent_config)?);

    let workflow_bodies = collect_workflow_bodies(ctx, &engine, &vars, &agent_config)?;
    for (workflow_id, workflow_body) in workflow_bodies {
        overlays.push(ManagedOverlay {
            path: workflow_source_path(&workflow_id),
            content: workflow_body.clone(),
        });

        for tool in &agent_config.tools {
            match tool.as_str() {
                "claude" => overlays.push(ManagedOverlay {
                    path: format!(".claude/commands/{workflow_id}.md"),
                    content: workflow_body.clone(),
                }),
                "opencode" => overlays.push(ManagedOverlay {
                    path: format!(".opencode/commands/{workflow_id}.md"),
                    content: workflow_body.clone(),
                }),
                "codex" => overlays.push(ManagedOverlay {
                    path: format!(".agents/skills/{workflow_id}/SKILL.md"),
                    content: render_codex_skill(&engine, &vars, &workflow_id, &workflow_body)?,
                }),
                _ => {}
            }
        }
    }

    if agent_config.tools.iter().any(|tool| tool == "codex") {
        overlays.push(ManagedOverlay {
            path: ".agents/codex/print-config.sh".into(),
            content: engine.render("agent/codex/print-config.sh", &vars)?,
        });
    }

    if agent_config.tools.iter().any(|tool| tool == "claude") {
        overlays.push(ManagedOverlay {
            path: ".claude/settings.json".into(),
            content: build_claude_settings(ctx, &agent_config),
        });
    }

    Ok(overlays)
}

fn collect_rule_overlays(
    engine: &TemplateEngine,
    vars: &StringMap,
    agent_config: &AgentConfig,
) -> Result<Vec<ManagedOverlay>> {
    let mut overlays = Vec::new();
    for tool in &agent_config.tools {
        match tool.as_str() {
            "cursor" => overlays.push(ManagedOverlay {
                path: ".cursor/rules".into(),
                content: engine.render("agent/cursor-rules", vars)?,
            }),
            "windsurf" => overlays.push(ManagedOverlay {
                path: ".windsurfrules".into(),
                content: engine.render("agent/windsurfrules", vars)?,
            }),
            "cline" => overlays.push(ManagedOverlay {
                path: ".clinerules".into(),
                content: engine.render("agent/clinerules", vars)?,
            }),
            "qoder" => overlays.push(ManagedOverlay {
                path: ".qoder/rules/harn.md".into(),
                content: engine.render("agent/qoderrules", vars)?,
            }),
            _ => {}
        }
    }

    Ok(overlays)
}

fn collect_workflow_bodies(
    ctx: &ProjectContext,
    engine: &TemplateEngine,
    vars: &StringMap,
    agent_config: &AgentConfig,
) -> Result<Vec<(String, String)>> {
    let mut workflows = Vec::new();

    for workflow_id in &agent_config.commands {
        let rendered = engine.render(&workflow_template_path(workflow_id), vars)?;
        let repo_path = ctx.path(&workflow_source_path(workflow_id));
        let workflow_body = if repo_path.exists() {
            std::fs::read_to_string(&repo_path).with_context(|| {
                format!("Failed to read workflow source: {}", repo_path.display())
            })?
        } else {
            rendered
        };
        workflows.push((workflow_id.clone(), workflow_body));
    }

    Ok(workflows)
}

fn render_codex_skill(
    engine: &TemplateEngine,
    vars: &StringMap,
    workflow_id: &str,
    workflow_body: &str,
) -> Result<String> {
    let workflow = agent_workflow(workflow_id)
        .with_context(|| format!("Unsupported agent workflow: {workflow_id}"))?;

    let mut skill_vars = vars.clone();
    skill_vars.insert("workflow_id".into(), workflow.id.into());
    skill_vars.insert("workflow_title".into(), workflow.skill_title.into());
    skill_vars.insert(
        "workflow_description".into(),
        workflow.skill_description.into(),
    );
    skill_vars.insert("workflow_command".into(), workflow.slash_command.into());
    skill_vars.insert("workflow_purpose".into(), workflow.purpose.into());
    skill_vars.insert("workflow_body".into(), workflow_body.into());

    engine.render("agent/skills/SKILL.md", &skill_vars)
}

pub fn build_slash_commands_table(commands: &[String]) -> String {
    commands
        .iter()
        .filter_map(|command| {
            agent_workflow(command)
                .map(|workflow| format!("| `{}` | {} |", workflow.slash_command, workflow.purpose))
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[must_use]
pub fn workflow_template_path(workflow_id: &str) -> String {
    format!("agent/workflows/{workflow_id}.md")
}

#[must_use]
pub fn workflow_source_path(workflow_id: &str) -> String {
    format!(".agents/workflows/{workflow_id}.md")
}

pub fn overlay_manifest_content(overlays: &[ManagedOverlay]) -> Result<String> {
    let manifest = AgentOverlayManifest::new(
        overlays
            .iter()
            .map(|overlay| overlay.path.clone())
            .collect::<Vec<_>>(),
    );
    Ok(serde_json::to_string_pretty(&manifest)?)
}

pub fn load_overlay_manifest(root: &Path) -> Result<Option<AgentOverlayManifest>> {
    let path = root.join(AGENT_OVERLAY_MANIFEST_PATH);
    if !path.exists() {
        return Ok(None);
    }

    let content = std::fs::read_to_string(&path)
        .with_context(|| format!("Failed to read overlay manifest: {}", path.display()))?;
    let manifest: AgentOverlayManifest =
        serde_json::from_str(&content).context("Failed to parse overlay manifest")?;
    Ok(Some(AgentOverlayManifest::new(manifest.artifacts)))
}

#[must_use]
pub fn stale_overlay_paths(
    previous: Option<&AgentOverlayManifest>,
    current: &BTreeSet<String>,
) -> Vec<String> {
    let Some(previous) = previous else {
        return Vec::new();
    };

    previous
        .artifact_set()
        .difference(current)
        .cloned()
        .collect()
}

pub fn expected_agent_overlays(root: &Path, config: &HarnConfig) -> Result<Vec<ManagedOverlay>> {
    let mut ctx = ProjectContext::new(root.to_path_buf(), config.clone());
    ctx.force = false;
    ctx.dry_run = true;
    collect_managed_overlays(&ctx)
}

type StringMap = HashMap<String, String>;

fn detect_package_manager(ctx: &ProjectContext) -> &'static str {
    if ctx.path("bun.lockb").exists() || ctx.path("bun.lock").exists() {
        "bun"
    } else if ctx.path("pnpm-lock.yaml").exists() {
        "pnpm"
    } else if ctx.path("yarn.lock").exists() {
        "yarn"
    } else {
        "npm"
    }
}

fn build_claude_settings(ctx: &ProjectContext, agent_config: &AgentConfig) -> String {
    let mut perms = vec![
        "Bash(make:*)".to_string(),
        "Bash(git:*)".to_string(),
        "Bash(gh:*)".to_string(),
        "Bash(ls:*)".to_string(),
        "Bash(curl:*)".to_string(),
        "Bash(docker:*)".to_string(),
    ];

    for lang in &ctx.config.stacks.languages {
        match lang.as_str() {
            "rust" => {
                perms.push("Bash(cargo:*)".into());
                perms.push("Bash(rustup:*)".into());
            }
            "go" => {
                perms.push("Bash(go:*)".into());
            }
            "typescript" | "javascript" => {
                let pkg = detect_package_manager(ctx);
                perms.push(format!("Bash({pkg}:*)"));
                match pkg {
                    "pnpm" => perms.push("Bash(pnpx:*)".into()),
                    "bun" => perms.push("Bash(bunx:*)".into()),
                    "yarn" => {}
                    _ => perms.push("Bash(npx:*)".into()),
                }
                perms.push("Bash(node:*)".into());
            }
            "dart" | "flutter" => {
                perms.push("Bash(flutter:*)".into());
                perms.push("Bash(dart:*)".into());
            }
            "python" => {
                perms.push("Bash(python3:*)".into());
                perms.push("Bash(pip:*)".into());
                perms.push("Bash(uv:*)".into());
            }
            "java" => {
                perms.push("Bash(java:*)".into());
                perms.push("Bash(javac:*)".into());
                perms.push("Bash(gradle:*)".into());
                perms.push("Bash(./gradlew:*)".into());
                perms.push("Bash(mvn:*)".into());
            }
            "cpp" | "c" => {
                perms.push("Bash(cmake:*)".into());
                perms.push("Bash(gcc:*)".into());
                perms.push("Bash(g++:*)".into());
                perms.push("Bash(clang:*)".into());
                perms.push("Bash(clang++:*)".into());
            }
            _ => {}
        }
    }

    if let Some(custom_perms) = agent_config.permissions.get("claude") {
        for permission in custom_perms {
            if !perms.contains(permission) {
                perms.push(permission.clone());
            }
        }
    }

    let perms_json: Vec<String> = perms
        .iter()
        .map(|perm| format!("      \"{perm}\""))
        .collect();
    let build_tool = ctx
        .config
        .modules
        .build
        .as_ref()
        .map_or("make", |build| build.tool.as_str());

    if agent_config.pre_commit_hook {
        let hook_cmd = format!("{build_tool} lint && {build_tool} test");
        format!(
            r#"{{
  "permissions": {{
    "allow": [
{}
    ]
  }},
  "hooks": {{
    "PreToolUse": [
      {{
        "matcher": "Bash(git commit*)",
        "hooks": [
          {{
            "type": "command",
            "command": "{hook_cmd}"
          }}
        ]
      }}
    ]
  }}
}}"#,
            perms_json.join(",\n")
        )
    } else {
        format!(
            r#"{{
  "permissions": {{
    "allow": [
{}
    ]
  }}
}}"#,
            perms_json.join(",\n")
        )
    }
}
