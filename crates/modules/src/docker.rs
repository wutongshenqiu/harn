use anyhow::Result;
use harn_core::context::ProjectContext;
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

    fn generate(&self, ctx: &mut ProjectContext) -> Result<Vec<String>> {
        let engine = TemplateEngine::new();
        let mut vars = TemplateEngine::vars_from_context(ctx);
        let force = ctx.force;
        let mut created = Vec::new();

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
                if engine.render_to(&src, &vars, &dst, force)? {
                    created.push("Dockerfile".into());
                }
            }
        }

        if docker_config.compose {
            let src = "docker/docker-compose.yml";
            if engine.has_template(src) {
                let dst = ctx.path("docker-compose.yml");
                if engine.render_to(src, &vars, &dst, force)? {
                    created.push("docker-compose.yml".into());
                }
            }
        }

        Ok(created)
    }
}
