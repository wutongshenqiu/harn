use anyhow::Result;
use harn_core::context::{ProjectContext, WriteStatus};
use harn_core::module::{Module, ModuleId};
use harn_templates::TemplateEngine;

/// Playbook template paths shipped with the SDD module.
/// Used by both generation (`SddModule`) and doctor checks (`sdd_checks`).
pub const SDD_PLAYBOOK_FILES: &[&str] = &[
    "sdd/playbooks/create-new-spec.md",
    "sdd/playbooks/coding-agent-workflow.md",
    "sdd/playbooks/write-prd-td.md",
    "sdd/playbooks/add-new-language.md",
];

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

    fn generate(&self, ctx: &mut ProjectContext) -> Result<Vec<(String, WriteStatus)>> {
        let engine = TemplateEngine::from_context(ctx);
        let vars = TemplateEngine::vars_from_context(ctx);
        let force = ctx.force;
        let mut files = Vec::new();

        // Spec templates (copy as-is, no variable substitution needed)
        let spec_templates = ["prd.md", "technical-design.md", "research.md"];
        for name in &spec_templates {
            let src = format!("sdd/specs/_templates/{name}");
            let dst = ctx.path(&format!("docs/specs/_templates/{name}"));
            let status = engine.copy_to(&src, &dst, force)?;
            files.push((format!("docs/specs/_templates/{name}"), status));
        }

        // Spec registry
        let dst = ctx.path("docs/specs/_index.md");
        let status = engine.render_to("sdd/specs/_index.md", &vars, &dst, force)?;
        files.push(("docs/specs/_index.md".into(), status));

        // Create active/completed directories
        if !ctx.dry_run {
            std::fs::create_dir_all(ctx.path("docs/specs/active"))?;
            std::fs::create_dir_all(ctx.path("docs/specs/completed"))?;
        }

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
                let status = engine.render_to(src, &vars, &dst, force)?;
                files.push((format!("docs/{rel}"), status));
            }
        }

        // Playbooks
        let include_playbooks = sdd_config.is_none_or(|c| c.playbooks);

        if include_playbooks {
            for src in SDD_PLAYBOOK_FILES {
                let rel = src.strip_prefix("sdd/").unwrap();
                let dst = ctx.path(&format!("docs/{rel}"));
                let status = engine.copy_to(src, &dst, force)?;
                files.push((format!("docs/{rel}"), status));
            }
        }

        // Docs README
        let dst = ctx.path("docs/README.md");
        let status = engine.render_to("sdd/README.md", &vars, &dst, force)?;
        files.push(("docs/README.md".into(), status));

        Ok(files)
    }
}
