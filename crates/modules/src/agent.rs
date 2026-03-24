use anyhow::Result;
use harn_core::agent_tools::validate_agent_tools;
use harn_core::context::{ProjectContext, WriteStatus};
use harn_core::module::{Module, ModuleId};

use crate::agent_overlay::{
    AGENT_OVERLAY_MANIFEST_PATH, collect_managed_overlays, load_overlay_manifest,
    overlay_manifest_content, stale_overlay_paths,
};

/// AI coding agent configuration module.
///
/// Supports multiple AI tools:
/// - Claude Code: `.claude/settings.json` + `.claude/commands/`
/// - Cursor: `.cursor/rules`
/// - Windsurf: `.windsurfrules`
/// - Cline: `.clinerules`
/// - `OpenCode`: `.opencode/commands/`
/// - Qoder: `.qoder/rules/`
/// - Codex: `.agents/workflows/` + `.agents/skills/`
///
/// Also generates `AGENTS.md` and `CLAUDE.md` project context files.
pub struct AgentModule;

impl Module for AgentModule {
    fn id(&self) -> ModuleId {
        "agent"
    }

    fn name(&self) -> &str {
        "AI Agent Config"
    }

    fn description(&self) -> &str {
        "AI coding agent configs (Claude, Cursor, Windsurf, Cline, OpenCode, Qoder, Codex)"
    }

    fn generate(&self, ctx: &mut ProjectContext) -> Result<Vec<(String, WriteStatus)>> {
        let agent_config = ctx.config.modules.agent.clone().unwrap_or_default();
        validate_agent_tools(&agent_config.tools)?;

        let previous_manifest = load_overlay_manifest(&ctx.root)?;
        let overlays = collect_managed_overlays(ctx)?;
        let current_paths = overlays
            .iter()
            .map(|overlay| overlay.path.clone())
            .collect::<std::collections::BTreeSet<_>>();

        let mut files = Vec::new();
        for overlay in &overlays {
            let dst = ctx.path(&overlay.path);
            let status = ctx.write_file(&dst, &overlay.content)?;
            files.push((overlay.path.clone(), status));
        }

        for stale_path in stale_overlay_paths(previous_manifest.as_ref(), &current_paths) {
            let status = ctx.delete_file(&ctx.path(&stale_path))?;
            if !matches!(status, WriteStatus::Skipped) {
                files.push((stale_path, status));
            }
        }

        let manifest_content = overlay_manifest_content(&overlays)?;
        let manifest_status =
            ctx.write_internal_file(&ctx.path(AGENT_OVERLAY_MANIFEST_PATH), &manifest_content)?;
        files.push((AGENT_OVERLAY_MANIFEST_PATH.into(), manifest_status));

        Ok(files)
    }
}
