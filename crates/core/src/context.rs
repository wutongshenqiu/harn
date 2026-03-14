use crate::config::HarnConfig;
use std::path::{Path, PathBuf};

/// Result of a file write operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WriteStatus {
    Created,
    Overwritten,
    Skipped,
    WouldCreate,
    WouldOverwrite,
}

impl WriteStatus {
    /// Whether the file was (or would be) written.
    pub fn is_written(self) -> bool {
        matches!(
            self,
            Self::Created | Self::Overwritten | Self::WouldCreate | Self::WouldOverwrite
        )
    }
}

/// Runtime context for module execution.
///
/// Provides access to the project directory, config, and utility methods.
#[derive(Debug, Clone)]
pub struct ProjectContext {
    /// Absolute path to the project root
    pub root: PathBuf,

    /// Project configuration
    pub config: HarnConfig,

    /// Whether to overwrite existing files
    pub force: bool,

    /// Preview mode — list files without writing
    pub dry_run: bool,

    /// Files that were created or modified during this run
    pub created_files: Vec<PathBuf>,
}

impl ProjectContext {
    pub fn new(root: PathBuf, config: HarnConfig) -> Self {
        Self {
            root,
            config,
            force: false,
            dry_run: false,
            created_files: Vec::new(),
        }
    }

    /// Get the project name.
    pub fn name(&self) -> &str {
        &self.config.project.name
    }

    /// Check if a language is in the stack.
    pub fn has_language(&self, lang: &str) -> bool {
        self.config
            .stacks
            .languages
            .iter()
            .any(|l| l.eq_ignore_ascii_case(lang))
    }

    /// Resolve a path relative to the project root.
    pub fn path(&self, relative: &str) -> PathBuf {
        self.root.join(relative)
    }

    /// Check if a file exists in the project.
    pub fn file_exists(&self, relative: &str) -> bool {
        self.path(relative).exists()
    }

    /// Record a file as created.
    pub fn record_created(&mut self, path: &Path) {
        self.created_files.push(path.to_path_buf());
    }

    /// Backup directory path: `<root>/.harn-backup/`
    fn backup_dir(&self) -> PathBuf {
        self.root.join(".harn-backup")
    }

    /// Copy an existing file into `.harn-backup/` before overwriting.
    fn backup_file(&self, path: &Path) -> std::io::Result<()> {
        let relative = path.strip_prefix(&self.root).unwrap_or(path);
        let backup_path = self.backup_dir().join(relative);
        if let Some(parent) = backup_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::copy(path, backup_path)?;
        Ok(())
    }

    /// Write content to a file, respecting force and `dry_run`.
    ///
    /// When `force` is set and the file exists, it is backed up to
    /// `.harn-backup/` before being overwritten.
    pub fn write_file(&self, path: &Path, content: &str) -> std::io::Result<WriteStatus> {
        let exists = path.exists();

        if exists && !self.force {
            return Ok(WriteStatus::Skipped);
        }
        if self.dry_run {
            return Ok(if exists {
                WriteStatus::WouldOverwrite
            } else {
                WriteStatus::WouldCreate
            });
        }
        if exists {
            self.backup_file(path)?;
        }
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, content)?;
        Ok(if exists {
            WriteStatus::Overwritten
        } else {
            WriteStatus::Created
        })
    }
}
