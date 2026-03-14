use anyhow::Result;
use harn_core::context::{ProjectContext, WriteStatus};
use harn_core::module::{Module, ModuleId};
use harn_templates::TemplateEngine;

/// Docker / containerization module.
pub struct DockerModule;

impl Module for DockerModule {
    fn id(&self) -> ModuleId {
        "docker"
    }

    fn name(&self) -> &str {
        "Docker"
    }

    fn description(&self) -> &str {
        "Dockerfile and docker-compose.yml"
    }

    fn generate(&self, ctx: &mut ProjectContext) -> Result<Vec<(String, WriteStatus)>> {
        let engine = TemplateEngine::from_context(ctx);
        let mut vars = TemplateEngine::vars_from_context(ctx);
        let force = ctx.force;
        let mut files = Vec::new();

        let primary_lang = ctx
            .config
            .stacks
            .languages
            .first()
            .cloned()
            .unwrap_or_else(|| "generic".into());
        vars.insert("primary_language".into(), primary_lang.clone());

        let docker_config = ctx.config.modules.docker.clone().unwrap_or_default();

        if docker_config.dockerfile {
            let src = format!("docker/Dockerfile.{primary_lang}");
            let src = if engine.has_template(&src) {
                src
            } else {
                "docker/Dockerfile.generic".into()
            };

            if engine.has_template(&src) {
                let dst = ctx.path("Dockerfile");
                let status = engine.render_to(&src, &vars, &dst, force)?;
                files.push(("Dockerfile".into(), status));
            }
        }

        if docker_config.compose {
            let src = "docker/docker-compose.yml";
            if engine.has_template(src) {
                let dst = ctx.path("docker-compose.yml");
                let status = engine.render_to(src, &vars, &dst, force)?;
                files.push(("docker-compose.yml".into(), status));
            }
        }

        Ok(files)
    }
}
