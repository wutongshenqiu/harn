use anyhow::{Context, Result};
use harn_core::ProjectContext;
use include_dir::{Dir, include_dir};
use minijinja::Environment;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
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
/// Cheap to construct — all state lives in a global `LazyLock`.
pub struct TemplateEngine;

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
        Self
    }

    /// Render a template with the given variables and write to the output path.
    ///
    /// If the output file exists and `force` is false, skip it.
    /// Returns `true` if the file was created/updated.
    pub fn render_to(
        &self,
        template_path: &str,
        vars: &HashMap<String, String>,
        output_path: &Path,
        force: bool,
    ) -> Result<bool> {
        if output_path.exists() && !force {
            return Ok(false);
        }

        let tmpl = TEMPLATE_ENV
            .get_template(template_path)
            .with_context(|| format!("Template not found: {template_path}"))?;

        let rendered = tmpl
            .render(minijinja::context! { ..minijinja::Value::from_serialize(vars) })
            .with_context(|| format!("Failed to render: {template_path}"))?;

        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(output_path, rendered)?;

        Ok(true)
    }

    /// Copy a template file without rendering (for non-template files).
    pub fn copy_to(&self, template_path: &str, output_path: &Path, force: bool) -> Result<bool> {
        if output_path.exists() && !force {
            return Ok(false);
        }

        let file = TEMPLATES_DIR
            .get_file(template_path)
            .with_context(|| format!("Template file not found: {template_path}"))?;

        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(output_path, file.contents())?;

        Ok(true)
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
        vars
    }
}
