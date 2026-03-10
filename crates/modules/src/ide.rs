use anyhow::Result;
use harn_core::context::ProjectContext;
use harn_core::module::{Module, ModuleId};
use harn_templates::TemplateEngine;

/// IDE / editor configuration module.
///
/// Supports:
/// - VS Code (.vscode/settings.json, extensions.json)
/// - Zed (.zed/settings.json)
/// - `JetBrains` (.idea/) — planned
/// - Vim/Neovim (.nvim.lua / .exrc) — planned
pub struct IdeModule;

impl Module for IdeModule {
    fn id(&self) -> ModuleId {
        "ide"
    }

    fn name(&self) -> &str {
        "IDE Configuration"
    }

    fn description(&self) -> &str {
        "Editor configs (VS Code, Zed; JetBrains, Vim planned)"
    }

    fn generate(&self, ctx: &mut ProjectContext) -> Result<Vec<String>> {
        let engine = TemplateEngine::with_dry_run(ctx.dry_run);
        let vars = TemplateEngine::vars_from_context(ctx);
        let force = ctx.force;
        let mut created = Vec::new();

        let ide_config = ctx.config.modules.ide.clone().unwrap_or_default();

        for editor in &ide_config.editors {
            match editor.as_str() {
                "vscode" => {
                    let files = [
                        ("ide/vscode/settings.json", ".vscode/settings.json"),
                        ("ide/vscode/extensions.json", ".vscode/extensions.json"),
                    ];
                    for (src, dst_rel) in &files {
                        if engine.has_template(src) {
                            let dst = ctx.path(dst_rel);
                            if engine.render_to(src, &vars, &dst, force)? {
                                created.push(dst_rel.to_string());
                            }
                        }
                    }
                }
                "zed" => {
                    let src = "ide/zed/settings.json";
                    if engine.has_template(src) {
                        let dst = ctx.path(".zed/settings.json");
                        if engine.render_to(src, &vars, &dst, force)? {
                            created.push(".zed/settings.json".into());
                        }
                    }
                }
                other => {
                    eprintln!(
                        "  {} No template for editor '{}', skipping",
                        console::style("WARN").yellow(),
                        other
                    );
                }
            }
        }

        Ok(created)
    }
}
