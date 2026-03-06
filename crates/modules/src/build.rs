use anyhow::Result;
use harn_core::context::ProjectContext;
use harn_core::module::{Module, ModuleId};
use harn_templates::TemplateEngine;

/// Build orchestration module.
///
/// Supports:
/// - make (Makefile)
/// - just (Justfile)
/// - task (Taskfile.yml)
pub struct BuildModule;

impl Module for BuildModule {
    fn id(&self) -> ModuleId {
        "build"
    }

    fn name(&self) -> &str {
        "Build Orchestration"
    }

    fn description(&self) -> &str {
        "Unified build tool (Make, Just, Task)"
    }

    fn generate(&self, ctx: &mut ProjectContext) -> Result<Vec<String>> {
        let engine = TemplateEngine::new();
        let mut vars = TemplateEngine::vars_from_context(ctx);
        let force = ctx.force;
        let mut created = Vec::new();

        let build_config = ctx.config.modules.build.clone().unwrap_or_default();
        let primary_lang = ctx
            .config
            .stacks
            .languages
            .first()
            .cloned()
            .unwrap_or_else(|| "generic".into());

        vars.insert("primary_language".into(), primary_lang.clone());

        let tool = build_config.tool.as_str();

        // Try language-specific template first, then generic
        let src_lang = format!("build/{tool}/{primary_lang}");
        let src_generic = format!("build/{tool}/generic");
        let src = if engine.has_template(&src_lang) {
            src_lang
        } else {
            src_generic
        };

        let output_file = match tool {
            "just" => "Justfile",
            "task" => "Taskfile.yml",
            _ => "Makefile",
        };

        let dst = ctx.path(output_file);
        if engine.has_template(&src) && engine.render_to(&src, &vars, &dst, force)? {
            created.push(output_file.into());
        }

        Ok(created)
    }
}
