use anyhow::Result;
use harn_core::context::ProjectContext;
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

    fn generate(&self, ctx: &mut ProjectContext) -> Result<Vec<String>> {
        let engine = TemplateEngine::new();
        let vars = TemplateEngine::vars_from_context(ctx);
        let force = ctx.force;
        let mut created = Vec::new();

        let src = "env/env.example";
        if engine.has_template(src) {
            let dst = ctx.path(".env.example");
            if engine.render_to(src, &vars, &dst, force)? {
                created.push(".env.example".into());
            }
        }

        Ok(created)
    }
}
