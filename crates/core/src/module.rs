use crate::context::{ProjectContext, WriteStatus};

/// Unique identifier for a module.
pub type ModuleId = &'static str;

/// A pluggable module that generates project infrastructure.
///
/// Modules are the core extension point of harn.
/// Each module owns a specific aspect of the project harness
/// (e.g., SDD docs, CI/CD, AI agent config, IDE settings).
///
/// # Design Principles (Harness Engineering)
///
/// - **Convention over configuration**: sensible defaults, escape hatches via config
/// - **Idempotent**: running twice produces the same result; skip existing files
/// - **Composable**: modules should not depend on each other's output
/// - **Self-documenting**: generated files include comments explaining their purpose
pub trait Module {
    /// Unique identifier (e.g., "sdd", "ci", "agent").
    fn id(&self) -> ModuleId;

    /// Human-readable name for display.
    fn name(&self) -> &str;

    /// Short description of what this module generates.
    fn description(&self) -> &str;

    /// Generate files into the project context.
    ///
    /// Returns the list of (path, status) pairs for each file attempted.
    fn generate(&self, ctx: &mut ProjectContext) -> anyhow::Result<Vec<(String, WriteStatus)>>;
}
