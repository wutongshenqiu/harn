use anyhow::Result;
use harn_core::context::{ProjectContext, WriteStatus};
use harn_core::module::{Module, ModuleId};
use harn_templates::TemplateEngine;

/// AI coding agent configuration module.
///
/// Supports multiple AI tools:
/// - Claude Code: .claude/settings.json + .claude/commands/
/// - Cursor: .cursor/rules
/// - Windsurf: .windsurfrules
/// - Cline: .clinerules
/// - `OpenCode`: .opencode/commands/
/// - Qoder: .qoder/rules/
///
/// Also generates CLAUDE.md and AGENTS.md project context files.
pub struct AgentModule;

impl Module for AgentModule {
    fn id(&self) -> ModuleId {
        "agent"
    }

    fn name(&self) -> &str {
        "AI Agent Config"
    }

    fn description(&self) -> &str {
        "AI coding agent configs (Claude, Cursor, Windsurf, Cline, OpenCode, Qoder)"
    }

    fn generate(&self, ctx: &mut ProjectContext) -> Result<Vec<(String, WriteStatus)>> {
        let engine = TemplateEngine::from_context(ctx);
        let mut vars = TemplateEngine::vars_from_context(ctx);
        let force = ctx.force;
        let mut files = Vec::new();

        let agent_config = ctx.config.modules.agent.clone().unwrap_or_default();

        // Build slash commands table from configured commands
        let slash_table = Self::build_slash_commands_table(&agent_config.commands);
        vars.insert("slash_commands_table".into(), slash_table);

        // CLAUDE.md and AGENTS.md — always generated (universal context)
        for name in &["CLAUDE.md", "AGENTS.md"] {
            let src = format!("agent/{name}");
            let dst = ctx.path(name);
            let status = engine.render_to(&src, &vars, &dst, force)?;
            files.push((name.to_string(), status));
        }

        // Generate per-tool configs
        for tool in &agent_config.tools {
            match tool.as_str() {
                "claude" => {
                    files.extend(self.generate_claude(ctx, &engine, &vars, &agent_config)?);
                }
                "cursor" => {
                    files.extend(self.generate_cursor(ctx, &engine, &vars)?);
                }
                "windsurf" => {
                    files.extend(self.generate_windsurf(ctx, &engine, &vars)?);
                }
                "cline" => {
                    files.extend(self.generate_cline(ctx, &engine, &vars)?);
                }
                "opencode" => {
                    files.extend(self.generate_opencode(ctx, &engine, &vars, &agent_config)?);
                }
                "qoder" => {
                    files.extend(self.generate_qoder(ctx, &engine, &vars)?);
                }
                _ => {} // Unknown tool, skip
            }
        }

        Ok(files)
    }
}

impl AgentModule {
    fn generate_claude(
        &self,
        ctx: &ProjectContext,
        engine: &TemplateEngine,
        vars: &std::collections::HashMap<String, String>,
        agent_config: &harn_core::config::AgentConfig,
    ) -> Result<Vec<(String, WriteStatus)>> {
        let force = ctx.force;
        let mut files = Vec::new();

        // settings.json — build dynamically based on stacks
        let settings = self.build_claude_settings(ctx, agent_config);
        let dst = ctx.path(".claude/settings.json");
        let status = ctx.write_file(&dst, &settings)?;
        files.push((".claude/settings.json".into(), status));

        // Slash commands
        for cmd_name in &agent_config.commands {
            let src = format!("agent/commands/{cmd_name}.md");
            if engine.has_template(&src) {
                let dst = ctx.path(&format!(".claude/commands/{cmd_name}.md"));
                let status = engine.render_to(&src, vars, &dst, force)?;
                files.push((format!(".claude/commands/{cmd_name}.md"), status));
            }
        }

        Ok(files)
    }

    fn generate_cursor(
        &self,
        ctx: &ProjectContext,
        engine: &TemplateEngine,
        vars: &std::collections::HashMap<String, String>,
    ) -> Result<Vec<(String, WriteStatus)>> {
        let mut files = Vec::new();
        let dst = ctx.path(".cursor/rules");
        let status = engine.render_to("agent/cursor-rules", vars, &dst, ctx.force)?;
        files.push((".cursor/rules".into(), status));
        Ok(files)
    }

    fn generate_windsurf(
        &self,
        ctx: &ProjectContext,
        engine: &TemplateEngine,
        vars: &std::collections::HashMap<String, String>,
    ) -> Result<Vec<(String, WriteStatus)>> {
        let mut files = Vec::new();
        let dst = ctx.path(".windsurfrules");
        let status = engine.render_to("agent/windsurfrules", vars, &dst, ctx.force)?;
        files.push((".windsurfrules".into(), status));
        Ok(files)
    }

    fn generate_cline(
        &self,
        ctx: &ProjectContext,
        engine: &TemplateEngine,
        vars: &std::collections::HashMap<String, String>,
    ) -> Result<Vec<(String, WriteStatus)>> {
        let mut files = Vec::new();
        let dst = ctx.path(".clinerules");
        let status = engine.render_to("agent/clinerules", vars, &dst, ctx.force)?;
        files.push((".clinerules".into(), status));
        Ok(files)
    }

    fn generate_opencode(
        &self,
        ctx: &ProjectContext,
        engine: &TemplateEngine,
        vars: &std::collections::HashMap<String, String>,
        agent_config: &harn_core::config::AgentConfig,
    ) -> Result<Vec<(String, WriteStatus)>> {
        let force = ctx.force;
        let mut files = Vec::new();

        for cmd_name in &agent_config.commands {
            let src = format!("agent/commands/{cmd_name}.md");
            if engine.has_template(&src) {
                let dst = ctx.path(&format!(".opencode/commands/{cmd_name}.md"));
                let status = engine.render_to(&src, vars, &dst, force)?;
                files.push((format!(".opencode/commands/{cmd_name}.md"), status));
            }
        }

        Ok(files)
    }

    fn generate_qoder(
        &self,
        ctx: &ProjectContext,
        engine: &TemplateEngine,
        vars: &std::collections::HashMap<String, String>,
    ) -> Result<Vec<(String, WriteStatus)>> {
        let mut files = Vec::new();
        let dst = ctx.path(".qoder/rules/harn.md");
        let status = engine.render_to("agent/qoderrules", vars, &dst, ctx.force)?;
        files.push((".qoder/rules/harn.md".into(), status));
        Ok(files)
    }

    fn build_slash_commands_table(commands: &[String]) -> String {
        let descriptions: &[(&str, &str, &str)] = &[
            ("ship", "/ship [msg]", "Lint + test + commit + push + PR"),
            ("implement", "/implement SPEC-NNN", "Implement a spec"),
            ("spec", "/spec create/list/advance", "Manage spec lifecycle"),
            ("lint", "/lint [fix]", "Run linters"),
            ("test", "/test [scope]", "Run tests"),
            ("review", "/review [PR#]", "Code review"),
            ("diagnose", "/diagnose [error]", "Diagnose issues"),
            ("deps", "/deps [check/update]", "Manage dependencies"),
            ("doc-audit", "/doc-audit", "Audit docs vs code"),
            ("issues", "/issues SPEC-NNN", "Generate issues from Spec"),
            ("retro", "/retro", "Session retrospective"),
            ("ci", "/ci [PR#]", "Check CI status"),
            ("pr", "/pr [title]", "Create pull request"),
            ("deploy", "/deploy", "Deploy"),
            ("sync-commands", "/sync-commands", "Sync slash commands"),
            (
                "run-plan",
                "/run-plan",
                "Orchestrate multi-spec long-running plans",
            ),
        ];

        let mut rows = Vec::new();
        for cmd in commands {
            if let Some((_, display, desc)) = descriptions.iter().find(|(id, _, _)| *id == cmd) {
                rows.push(format!("| `{display}` | {desc} |"));
            } else {
                rows.push(format!("| `/{cmd}` | {cmd} |"));
            }
        }
        rows.join("\n")
    }

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

    fn build_claude_settings(
        &self,
        ctx: &ProjectContext,
        agent_config: &harn_core::config::AgentConfig,
    ) -> String {
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
                    let pkg = Self::detect_package_manager(ctx);
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

        // Also check custom permissions from config
        if let Some(agent_cfg) = &ctx.config.modules.agent {
            if let Some(custom_perms) = agent_cfg.permissions.get("claude") {
                for p in custom_perms {
                    if !perms.contains(p) {
                        perms.push(p.clone());
                    }
                }
            }
        }

        let perms_json: Vec<String> = perms.iter().map(|p| format!("      \"{p}\"")).collect();

        if agent_config.pre_commit_hook {
            let hook_cmd = "make lint && make test";
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
}
