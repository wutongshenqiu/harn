use anyhow::Result;
use harn_core::context::ProjectContext;
use harn_core::module::{Module, ModuleId};
use harn_templates::TemplateEngine;

/// Code quality tooling module.
///
/// Generates .editorconfig and language-specific linter configs.
pub struct QualityModule;

impl Module for QualityModule {
    fn id(&self) -> ModuleId {
        "quality"
    }

    fn name(&self) -> &str {
        "Code Quality"
    }

    fn description(&self) -> &str {
        "EditorConfig, linter configs"
    }

    fn generate(&self, ctx: &mut ProjectContext) -> Result<Vec<String>> {
        let engine = TemplateEngine::new();
        let vars = TemplateEngine::vars_from_context(ctx);
        let force = ctx.force;
        let mut created = Vec::new();

        let quality_config = ctx.config.modules.quality.clone().unwrap_or_default();

        if quality_config.editorconfig {
            let src = "quality/editorconfig";
            if engine.has_template(src) {
                let dst = ctx.path(".editorconfig");
                if engine.render_to(src, &vars, &dst, force)? {
                    created.push(".editorconfig".into());
                }
            }
        }

        // Language-specific quality configs
        for lang in &ctx.config.stacks.languages {
            match lang.as_str() {
                "go" => {
                    let src = "quality/golangci.yml";
                    if engine.has_template(src) {
                        let dst = ctx.path(".golangci.yml");
                        if engine.render_to(src, &vars, &dst, force)? {
                            created.push(".golangci.yml".into());
                        }
                    }
                }
                "rust" => {
                    let src = "quality/rust-toolchain.toml";
                    if engine.has_template(src) {
                        let dst = ctx.path("rust-toolchain.toml");
                        if engine.render_to(src, &vars, &dst, force)? {
                            created.push("rust-toolchain.toml".into());
                        }
                    }
                }
                "typescript" | "javascript" => {
                    for (src, dst_rel) in &[
                        ("quality/eslint.config.js", "eslint.config.js"),
                        ("quality/prettierrc", ".prettierrc"),
                    ] {
                        if engine.has_template(src) {
                            let dst = ctx.path(dst_rel);
                            if engine.render_to(src, &vars, &dst, force)? {
                                created.push(dst_rel.to_string());
                            }
                        }
                    }
                }
                "python" => {
                    let src = "quality/ruff.toml";
                    if engine.has_template(src) {
                        let dst = ctx.path("ruff.toml");
                        if engine.render_to(src, &vars, &dst, force)? {
                            created.push("ruff.toml".into());
                        }
                    }
                }
                "java" => {
                    let src = "quality/checkstyle.xml";
                    if engine.has_template(src) {
                        let dst = ctx.path("checkstyle.xml");
                        if engine.render_to(src, &vars, &dst, force)? {
                            created.push("checkstyle.xml".into());
                        }
                    }
                }
                _ => {}
            }
        }

        Ok(created)
    }
}
