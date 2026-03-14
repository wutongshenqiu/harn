use anyhow::{Context, Result};
use harn_core::ProjectContext;
use harn_core::context::WriteStatus;
use include_dir::{Dir, include_dir};
use minijinja::Environment;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

/// All templates are embedded at compile time from the workspace `templates/` dir.
static TEMPLATES_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/../../templates");

/// Global template environment — initialized once, reused across all modules.
static TEMPLATE_ENV: LazyLock<Environment<'static>> = LazyLock::new(|| {
    let mut env = Environment::new();
    for entry in TEMPLATES_DIR.find("**/*").unwrap() {
        if let Some(file) = entry.as_file() {
            let path = file.path().to_str().unwrap();
            let content = file.contents_utf8().unwrap_or("");
            // Leak is fine — static templates live for the entire process.
            let path_owned: &'static str = Box::leak(path.to_string().into_boxed_str());
            let content_owned: &'static str = Box::leak(content.to_string().into_boxed_str());
            env.add_template(path_owned, content_owned).ok();
        }
    }
    env
});

/// Template rendering engine backed by minijinja + embedded templates.
///
/// When `dry_run` is true, templates are rendered (to validate them) but
/// files are not written to disk. When `backup_root` is set, existing files
/// are backed up before being overwritten.
pub struct TemplateEngine {
    dry_run: bool,
    backup_root: Option<PathBuf>,
}

impl Default for TemplateEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl TemplateEngine {
    #[must_use]
    pub fn new() -> Self {
        // Force initialization on first use
        LazyLock::force(&TEMPLATE_ENV);
        Self {
            dry_run: false,
            backup_root: None,
        }
    }

    /// Create an engine configured from a `ProjectContext`.
    #[must_use]
    pub fn from_context(ctx: &ProjectContext) -> Self {
        LazyLock::force(&TEMPLATE_ENV);
        let backup_root = if ctx.force {
            Some(ctx.root.clone())
        } else {
            None
        };
        Self {
            dry_run: ctx.dry_run,
            backup_root,
        }
    }

    /// Create an engine in dry-run mode (no file writes).
    #[must_use]
    pub fn with_dry_run(dry_run: bool) -> Self {
        LazyLock::force(&TEMPLATE_ENV);
        Self {
            dry_run,
            backup_root: None,
        }
    }

    /// Backup an existing file to `.harn-backup/` before overwriting.
    fn backup_file(&self, output_path: &Path) -> std::io::Result<()> {
        let Some(root) = &self.backup_root else {
            return Ok(());
        };
        let relative = output_path.strip_prefix(root).unwrap_or(output_path);
        let backup_path = root.join(".harn-backup").join(relative);
        if let Some(parent) = backup_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::copy(output_path, backup_path)?;
        Ok(())
    }

    /// Render a template with the given variables and write to the output path.
    ///
    /// If the output file exists and `force` is false, skip it.
    /// Returns `WriteStatus` indicating what happened.
    pub fn render_to(
        &self,
        template_path: &str,
        vars: &HashMap<String, String>,
        output_path: &Path,
        force: bool,
    ) -> Result<WriteStatus> {
        let exists = output_path.exists();

        if exists && !force {
            return Ok(WriteStatus::Skipped);
        }

        let tmpl = TEMPLATE_ENV
            .get_template(template_path)
            .with_context(|| format!("Template not found: {template_path}"))?;

        let rendered = tmpl
            .render(minijinja::context! { ..minijinja::Value::from_serialize(vars) })
            .with_context(|| format!("Failed to render: {template_path}"))?;

        if self.dry_run {
            return Ok(if exists {
                WriteStatus::WouldOverwrite
            } else {
                WriteStatus::WouldCreate
            });
        }

        if exists {
            self.backup_file(output_path)?;
        }
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(output_path, rendered)?;

        Ok(if exists {
            WriteStatus::Overwritten
        } else {
            WriteStatus::Created
        })
    }

    /// Copy a template file without rendering (for non-template files).
    pub fn copy_to(
        &self,
        template_path: &str,
        output_path: &Path,
        force: bool,
    ) -> Result<WriteStatus> {
        let exists = output_path.exists();

        if exists && !force {
            return Ok(WriteStatus::Skipped);
        }

        let file = TEMPLATES_DIR
            .get_file(template_path)
            .with_context(|| format!("Template file not found: {template_path}"))?;

        if self.dry_run {
            return Ok(if exists {
                WriteStatus::WouldOverwrite
            } else {
                WriteStatus::WouldCreate
            });
        }

        if exists {
            self.backup_file(output_path)?;
        }
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(output_path, file.contents())?;

        Ok(if exists {
            WriteStatus::Overwritten
        } else {
            WriteStatus::Created
        })
    }

    /// Check if a template exists.
    pub fn has_template(&self, path: &str) -> bool {
        TEMPLATES_DIR.get_file(path).is_some()
    }

    /// List all template files under a directory prefix.
    pub fn list_templates(&self, prefix: &str) -> Vec<String> {
        let mut result = Vec::new();
        for entry in TEMPLATES_DIR.find("**/*").unwrap() {
            if let Some(file) = entry.as_file() {
                let path = file.path().to_str().unwrap();
                if path.starts_with(prefix) {
                    result.push(path.to_string());
                }
            }
        }
        result
    }

    /// Get raw bytes of an embedded template file.
    pub fn get_embedded_content(template_path: &str) -> Option<&'static [u8]> {
        TEMPLATES_DIR
            .get_file(template_path)
            .map(include_dir::File::contents)
    }

    /// Build standard template variables from a project context.
    pub fn vars_from_context(ctx: &ProjectContext) -> HashMap<String, String> {
        let mut vars = HashMap::new();
        vars.insert("project_name".into(), ctx.config.project.name.clone());
        vars.insert("project_type".into(), ctx.config.project.r#type.clone());
        vars.insert("languages".into(), ctx.config.stacks.languages.join(", "));
        vars.insert("frameworks".into(), ctx.config.stacks.frameworks.join(", "));
        vars.insert("year".into(), harn_core::date::year());
        vars.insert("today".into(), harn_core::date::today());

        // Build tool
        let build_tool = ctx
            .config
            .modules
            .build
            .as_ref()
            .map_or("make", |b| b.tool.as_str())
            .to_string();
        vars.insert("build_tool".into(), build_tool);

        // CI provider
        if let Some(ci) = &ctx.config.modules.ci {
            vars.insert("ci_provider".into(), ci.provider.clone());
        }

        vars
    }
}
