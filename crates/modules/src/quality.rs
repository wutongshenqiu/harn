use anyhow::Result;
use harn_core::context::{ProjectContext, WriteStatus};
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

    fn generate(&self, ctx: &mut ProjectContext) -> Result<Vec<(String, WriteStatus)>> {
        let engine = TemplateEngine::from_context(ctx);
        let vars = TemplateEngine::vars_from_context(ctx);
        let force = ctx.force;
        let mut files = Vec::new();

        let quality_config = ctx.config.modules.quality.clone().unwrap_or_default();

        if quality_config.editorconfig {
            let src = "quality/editorconfig";
            if engine.has_template(src) {
                let dst = ctx.path(".editorconfig");
                let status = engine.render_to(src, &vars, &dst, force)?;
                files.push((".editorconfig".into(), status));
            }
        }

        // Language-specific quality configs
        for lang in &ctx.config.stacks.languages {
            match lang.as_str() {
                "go" => {
                    let src = "quality/golangci.yml";
                    if engine.has_template(src) {
                        let dst = ctx.path(".golangci.yml");
                        let status = engine.render_to(src, &vars, &dst, force)?;
                        files.push((".golangci.yml".into(), status));
                    }
                }
                "rust" => {
                    let src = "quality/rust-toolchain.toml";
                    if engine.has_template(src) {
                        let dst = ctx.path("rust-toolchain.toml");
                        let status = engine.render_to(src, &vars, &dst, force)?;
                        files.push(("rust-toolchain.toml".into(), status));
                    }
                }
                "typescript" | "javascript" => {
                    for (src, dst_rel) in &[
                        ("quality/eslint.config.js", "eslint.config.js"),
                        ("quality/prettierrc", ".prettierrc"),
                    ] {
                        if engine.has_template(src) {
                            let dst = ctx.path(dst_rel);
                            let status = engine.render_to(src, &vars, &dst, force)?;
                            files.push((dst_rel.to_string(), status));
                        }
                    }
                }
                "python" => {
                    let src = "quality/ruff.toml";
                    if engine.has_template(src) {
                        let dst = ctx.path("ruff.toml");
                        let status = engine.render_to(src, &vars, &dst, force)?;
                        files.push(("ruff.toml".into(), status));
                    }
                }
                "java" => {
                    let src = "quality/checkstyle.xml";
                    if engine.has_template(src) {
                        let dst = ctx.path("checkstyle.xml");
                        let status = engine.render_to(src, &vars, &dst, force)?;
                        files.push(("checkstyle.xml".into(), status));
                    }
                }
                "cpp" | "c" => {
                    let src = "quality/clang-format";
                    if engine.has_template(src) {
                        let dst = ctx.path(".clang-format");
                        let status = engine.render_to(src, &vars, &dst, force)?;
                        files.push((".clang-format".into(), status));
                    }
                }
                _ => {}
            }
        }

        Ok(files)
    }
}
