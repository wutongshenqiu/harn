use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::Path;

/// Root configuration for a harn project.
///
/// Can be loaded from `harn.toml` or constructed interactively.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarnConfig {
    pub project: ProjectConfig,

    #[serde(default)]
    pub stacks: StacksConfig,

    #[serde(default)]
    pub modules: ModulesConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub name: String,

    /// single | monorepo
    #[serde(default = "default_project_type")]
    pub r#type: String,
}

fn default_project_type() -> String {
    "single".into()
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StacksConfig {
    /// Primary languages: rust, go, typescript, dart, python, etc.
    #[serde(default)]
    pub languages: Vec<String>,

    /// Frameworks: axum, chi, react, flutter, nextjs, etc.
    #[serde(default)]
    pub frameworks: Vec<String>,
}

/// Module selection and configuration.
///
/// Each module is optional and can be customized.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ModulesConfig {
    /// SDD (Spec-Driven Development) documentation structure
    #[serde(default)]
    pub sdd: Option<SddConfig>,

    /// CI/CD pipeline configuration
    #[serde(default)]
    pub ci: Option<CiConfig>,

    /// AI coding agent configuration
    #[serde(default)]
    pub agent: Option<AgentConfig>,

    /// Build orchestration tool
    #[serde(default)]
    pub build: Option<BuildConfig>,

    /// IDE / editor configuration
    #[serde(default)]
    pub ide: Option<IdeConfig>,

    /// Git configuration (hooks, ignore, attributes)
    #[serde(default)]
    pub git: Option<GitConfig>,

    /// Docker / containerization
    #[serde(default)]
    pub docker: Option<DockerConfig>,

    /// Environment variable management
    #[serde(default)]
    pub env: Option<EnvConfig>,

    /// Code quality tooling (linters, formatters, pre-commit)
    #[serde(default)]
    pub quality: Option<QualityConfig>,
}

// ---------------------------------------------------------------------------
// Module-specific configs
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SddConfig {
    /// Include playbooks
    #[serde(default = "default_true")]
    pub playbooks: bool,

    /// Include reference docs (types, api, data-model)
    #[serde(default = "default_true")]
    pub reference: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CiConfig {
    /// CI platform: github, gitlab, codeberg
    #[serde(default = "default_ci_provider")]
    pub provider: String,

    /// Which workflows to generate
    #[serde(default = "default_ci_workflows")]
    pub workflows: Vec<String>,
}

fn default_ci_provider() -> String {
    "github".into()
}

fn default_ci_workflows() -> Vec<String> {
    vec!["ci".into(), "cd".into()]
}

impl Default for CiConfig {
    fn default() -> Self {
        Self {
            provider: default_ci_provider(),
            workflows: default_ci_workflows(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    /// AI tools to configure: claude, cursor, windsurf, cline, opencode
    #[serde(default = "default_agent_tools")]
    pub tools: Vec<String>,

    /// Which slash commands to include
    #[serde(default = "default_commands")]
    pub commands: Vec<String>,

    /// Enable pre-commit hook (lint + test)
    #[serde(default = "default_true")]
    pub pre_commit_hook: bool,

    /// Custom permissions for each tool (tool -> list of allowed commands)
    #[serde(default)]
    pub permissions: BTreeMap<String, Vec<String>>,
}

fn default_agent_tools() -> Vec<String> {
    vec!["claude".into()]
}

fn default_commands() -> Vec<String> {
    vec![
        "ship".into(),
        "implement".into(),
        "spec".into(),
        "lint".into(),
        "test".into(),
        "review".into(),
        "diagnose".into(),
        "deps".into(),
        "issues".into(),
        "doc-audit".into(),
        "retro".into(),
        "sync-commands".into(),
        "ci".into(),
        "pr".into(),
        "deploy".into(),
    ]
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            tools: default_agent_tools(),
            commands: default_commands(),
            pre_commit_hook: true,
            permissions: BTreeMap::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildConfig {
    /// Build tool: make, just, task
    #[serde(default = "default_build_tool")]
    pub tool: String,
}

fn default_build_tool() -> String {
    "make".into()
}

impl Default for BuildConfig {
    fn default() -> Self {
        Self {
            tool: default_build_tool(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdeConfig {
    /// Editors: vscode, jetbrains, vim, zed
    #[serde(default = "default_editors")]
    pub editors: Vec<String>,
}

fn default_editors() -> Vec<String> {
    vec!["vscode".into()]
}

impl Default for IdeConfig {
    fn default() -> Self {
        Self {
            editors: default_editors(),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GitConfig {
    /// Generate .gitignore
    #[serde(default = "default_true")]
    pub gitignore: bool,

    /// Generate .gitattributes
    #[serde(default)]
    pub gitattributes: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DockerConfig {
    /// Generate Dockerfile
    #[serde(default = "default_true")]
    pub dockerfile: bool,

    /// Generate docker-compose.yml
    #[serde(default = "default_true")]
    pub compose: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EnvConfig {
    /// Extra environment variable names
    #[serde(default)]
    pub extra_vars: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QualityConfig {
    /// Pre-commit hook command
    #[serde(default)]
    pub pre_commit_cmd: Option<String>,

    /// `EditorConfig`
    #[serde(default = "default_true")]
    pub editorconfig: bool,
}

fn default_true() -> bool {
    true
}

impl HarnConfig {
    pub fn load(path: &Path) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: Self = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn save(&self, path: &Path) -> anyhow::Result<()> {
        let content = toml::to_string_pretty(self)?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, content)?;
        Ok(())
    }

    /// Create a default config with all modules enabled.
    pub fn default_all(name: String) -> Self {
        Self {
            project: ProjectConfig {
                name,
                r#type: "single".into(),
            },
            stacks: StacksConfig::default(),
            modules: ModulesConfig {
                sdd: Some(SddConfig::default()),
                ci: Some(CiConfig::default()),
                agent: Some(AgentConfig::default()),
                build: Some(BuildConfig::default()),
                ide: Some(IdeConfig::default()),
                git: Some(GitConfig::default()),
                docker: Some(DockerConfig::default()),
                env: Some(EnvConfig::default()),
                quality: Some(QualityConfig::default()),
            },
        }
    }

    /// Get list of enabled module IDs.
    pub fn enabled_modules(&self) -> Vec<String> {
        let mut modules = Vec::new();
        if self.modules.sdd.is_some() {
            modules.push("sdd".into());
        }
        if self.modules.ci.is_some() {
            modules.push("ci".into());
        }
        if self.modules.agent.is_some() {
            modules.push("agent".into());
        }
        if self.modules.build.is_some() {
            modules.push("build".into());
        }
        if self.modules.ide.is_some() {
            modules.push("ide".into());
        }
        if self.modules.git.is_some() {
            modules.push("git".into());
        }
        if self.modules.docker.is_some() {
            modules.push("docker".into());
        }
        if self.modules.env.is_some() {
            modules.push("env".into());
        }
        if self.modules.quality.is_some() {
            modules.push("quality".into());
        }
        modules
    }
}

impl Default for SddConfig {
    fn default() -> Self {
        Self {
            playbooks: true,
            reference: true,
        }
    }
}
