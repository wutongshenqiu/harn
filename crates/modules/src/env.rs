use anyhow::Result;
use harn_core::context::{ProjectContext, WriteStatus};
use harn_core::module::{Module, ModuleId};
use harn_templates::TemplateEngine;

/// Environment variable management module.
pub struct EnvModule;

impl Module for EnvModule {
    fn id(&self) -> ModuleId {
        "env"
    }

    fn name(&self) -> &str {
        "Environment"
    }

    fn description(&self) -> &str {
        ".env.example template"
    }

    fn generate(&self, ctx: &mut ProjectContext) -> Result<Vec<(String, WriteStatus)>> {
        let engine = TemplateEngine::from_context(ctx);
        let mut vars = TemplateEngine::vars_from_context(ctx);
        let force = ctx.force;
        let mut files = Vec::new();

        let env_config = ctx.config.modules.env.clone().unwrap_or_default();

        // Pass extra_vars as a newline-separated string for template rendering
        let extra = env_config
            .extra_vars
            .iter()
            .map(|v| format!("# {v}="))
            .collect::<Vec<_>>()
            .join("\n");
        vars.insert("extra_vars".into(), extra);

        let src = "env/env.example";
        if engine.has_template(src) {
            let dst = ctx.path(".env.example");
            let status = engine.render_to(src, &vars, &dst, force)?;
            files.push((".env.example".into(), status));
        }

        Ok(files)
    }
}
