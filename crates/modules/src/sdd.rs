use anyhow::Result;
use harn_core::context::ProjectContext;
use harn_core::module::{Module, ModuleId};
use harn_templates::TemplateEngine;

/// Spec-Driven Development documentation structure.
///
/// Generates:
/// - docs/specs/ (registry, templates, active/completed dirs)
/// - docs/reference/ (types, API conventions, data model)
/// - docs/playbooks/ (how-to guides)
pub struct SddModule;

impl Module for SddModule {
    fn id(&self) -> ModuleId {
        "sdd"
    }

    fn name(&self) -> &str {
        "SDD (Spec-Driven Development)"
    }

    fn description(&self) -> &str {
        "Documentation structure: specs, reference docs, playbooks"
    }

    fn generate(&self, ctx: &mut ProjectContext) -> Result<Vec<String>> {
        let engine = TemplateEngine::new();
        let vars = TemplateEngine::vars_from_context(ctx);
        let force = ctx.force;
        let mut created = Vec::new();

        // Spec templates (copy as-is, no variable substitution needed)
        let spec_templates = ["prd.md", "technical-design.md", "research.md"];
        for name in &spec_templates {
            let src = format!("sdd/specs/_templates/{name}");
            let dst = ctx.path(&format!("docs/specs/_templates/{name}"));
            if engine.copy_to(&src, &dst, force)? {
                created.push(format!("docs/specs/_templates/{name}"));
            }
        }

        // Spec registry
        let dst = ctx.path("docs/specs/_index.md");
        if engine.render_to("sdd/specs/_index.md", &vars, &dst, force)? {
            created.push("docs/specs/_index.md".into());
        }

        // Create active/completed directories
        std::fs::create_dir_all(ctx.path("docs/specs/active"))?;
        std::fs::create_dir_all(ctx.path("docs/specs/completed"))?;

        // Reference docs
        let sdd_config = ctx.config.modules.sdd.as_ref();
        let include_reference = sdd_config.is_none_or(|c| c.reference);

        if include_reference {
            let ref_templates = [
                "sdd/reference/api-conventions.md",
                "sdd/reference/data-model.md",
                "sdd/reference/types/enums.md",
                "sdd/reference/types/models.md",
                "sdd/reference/types/api-dtos.md",
            ];

            for src in &ref_templates {
                let rel = src.strip_prefix("sdd/").unwrap();
                let dst = ctx.path(&format!("docs/{rel}"));
                if engine.render_to(src, &vars, &dst, force)? {
                    created.push(format!("docs/{rel}"));
                }
            }
        }

        // Playbooks
        let include_playbooks = sdd_config.is_none_or(|c| c.playbooks);

        if include_playbooks {
            let playbook_files = [
                "sdd/playbooks/create-new-spec.md",
                "sdd/playbooks/coding-agent-workflow.md",
            ];

            for src in &playbook_files {
                let rel = src.strip_prefix("sdd/").unwrap();
                let dst = ctx.path(&format!("docs/{rel}"));
                if engine.copy_to(src, &dst, force)? {
                    created.push(format!("docs/{rel}"));
                }
            }
        }

        // Docs README
        let dst = ctx.path("docs/README.md");
        if engine.render_to("sdd/README.md", &vars, &dst, force)? {
            created.push("docs/README.md".into());
        }

        Ok(created)
    }
}
