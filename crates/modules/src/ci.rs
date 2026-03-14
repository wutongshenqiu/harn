use anyhow::Result;
use harn_core::context::{ProjectContext, WriteStatus};
use harn_core::module::{Module, ModuleId};
use harn_templates::TemplateEngine;

/// CI/CD pipeline configuration.
///
/// Supports multiple providers:
/// - GitHub Actions (.github/workflows/)
/// - GitLab CI (.gitlab-ci.yml)
/// - Gitea/Codeberg (.gitea/workflows/)
pub struct CiModule;

impl Module for CiModule {
    fn id(&self) -> ModuleId {
        "ci"
    }

    fn name(&self) -> &str {
        "CI/CD Pipelines"
    }

    fn description(&self) -> &str {
        "CI/CD workflows (GitHub Actions, GitLab CI, Gitea)"
    }

    fn generate(&self, ctx: &mut ProjectContext) -> Result<Vec<(String, WriteStatus)>> {
        let engine = TemplateEngine::from_context(ctx);
        let mut vars = TemplateEngine::vars_from_context(ctx);
        let force = ctx.force;
        let mut files = Vec::new();

        let ci_config = ctx.config.modules.ci.clone().unwrap_or_default();

        // Add language-specific variables for CI templates
        let primary_lang = ctx
            .config
            .stacks
            .languages
            .first()
            .cloned()
            .unwrap_or_default();
        vars.insert("primary_language".into(), primary_lang);

        let provider = ci_config.provider.as_str();

        for workflow in &ci_config.workflows {
            let src = format!("ci/{provider}/{workflow}.yml");

            // Determine output path based on provider
            let output_rel = match provider {
                "github" => format!(".github/workflows/{workflow}.yml"),
                "gitlab" => {
                    if workflow == "ci" {
                        ".gitlab-ci.yml".to_string()
                    } else {
                        format!(".gitlab/{workflow}.yml")
                    }
                }
                "gitea" | "codeberg" => format!(".gitea/workflows/{workflow}.yml"),
                _ => format!(".ci/{workflow}.yml"),
            };

            let dst = ctx.path(&output_rel);

            if engine.has_template(&src) {
                let status = engine.render_to(&src, &vars, &dst, force)?;
                files.push((output_rel, status));
            }
        }

        // Claude action workflow (special case — only for GitHub)
        if provider == "github" {
            let src = "ci/github/claude.yml";
            if engine.has_template(src) {
                let dst = ctx.path(".github/workflows/claude.yml");
                let status = engine.render_to(src, &vars, &dst, force)?;
                files.push((".github/workflows/claude.yml".into(), status));
            }
        }

        Ok(files)
    }
}
