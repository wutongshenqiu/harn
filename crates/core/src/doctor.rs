use std::fmt;
use std::path::Path;

/// Diagnostic severity level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    Info,
    Warning,
    Error,
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Info => write!(f, "info"),
            Self::Warning => write!(f, "warn"),
            Self::Error => write!(f, "err"),
        }
    }
}

/// An automatically applicable fix.
#[derive(Debug, Clone)]
pub enum AutoFix {
    /// Add a missing entry to `_index.md`.
    AddRegistryEntry {
        spec_id: String,
        title: String,
        status: String,
        location: String,
    },
    /// Remove a registry entry whose directory does not exist.
    RemoveRegistryEntry { spec_id: String },
    /// Update a field in a registry entry.
    UpdateRegistryEntry {
        spec_id: String,
        old_line: String,
        new_line: String,
    },
    /// Create a missing directory.
    CreateDirectory { path: String },
    /// Overwrite a template file with the built-in version.
    UpdateTemplate { path: String, content: Vec<u8> },
}

/// A single diagnostic finding.
#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub severity: Severity,
    pub check: String,
    pub message: String,
    pub fix: Option<AutoFix>,
}

/// Aggregated check results.
#[derive(Debug, Default)]
pub struct CheckResult {
    pub diagnostics: Vec<Diagnostic>,
}

impl CheckResult {
    pub fn ok(check: &str, message: impl Into<String>) {
        // Prints inline during check execution
        println!("  {}  {}", console::style("ok").green(), message.into(),);
        let _ = check; // used for context
    }

    pub fn push(&mut self, d: Diagnostic) {
        let icon = match d.severity {
            Severity::Info => console::style("info").blue(),
            Severity::Warning => console::style("warn").yellow(),
            Severity::Error => console::style("err ").red(),
        };
        let fixable = if d.fix.is_some() { " [fixable]" } else { "" };
        println!("  {icon}  {}{fixable}", d.message);
        self.diagnostics.push(d);
    }

    pub fn merge(&mut self, other: Self) {
        self.diagnostics.extend(other.diagnostics);
    }

    pub fn has_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|d| d.severity == Severity::Error)
    }

    pub fn has_warnings(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|d| d.severity == Severity::Warning)
    }

    pub fn has_fixable(&self) -> bool {
        self.diagnostics.iter().any(|d| d.fix.is_some())
    }

    /// Exit code: 0 = clean, 1 = warnings only, 2 = errors.
    pub fn exit_code(&self) -> i32 {
        if self.has_errors() {
            2
        } else {
            i32::from(self.has_warnings())
        }
    }

    pub fn error_count(&self) -> usize {
        self.diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Error)
            .count()
    }

    pub fn warning_count(&self) -> usize {
        self.diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Warning)
            .count()
    }
}

/// Print a summary line after all checks.
pub fn print_summary(result: &CheckResult) {
    let errors = result.error_count();
    let warnings = result.warning_count();
    let fixable = result
        .diagnostics
        .iter()
        .filter(|d| d.fix.is_some())
        .count();

    println!();
    if errors == 0 && warnings == 0 {
        println!("{}", console::style("All checks passed.").green().bold());
    } else {
        let mut parts = Vec::new();
        if errors > 0 {
            parts.push(format!(
                "{}",
                console::style(format!(
                    "{errors} error{}",
                    if errors == 1 { "" } else { "s" }
                ))
                .red()
                .bold()
            ));
        }
        if warnings > 0 {
            parts.push(format!(
                "{}",
                console::style(format!(
                    "{warnings} warning{}",
                    if warnings == 1 { "" } else { "s" }
                ))
                .yellow()
                .bold()
            ));
        }
        println!("Result: {}", parts.join(", "));

        if fixable > 0 {
            println!(
                "Run {} to auto-fix {fixable} issue{}.",
                console::style("harn doctor --fix").cyan(),
                if fixable == 1 { "" } else { "s" }
            );
        }
    }
}

/// Execute all auto-fixes from the diagnostics.
pub fn apply_fixes(root: &Path, result: &CheckResult) -> anyhow::Result<Vec<String>> {
    let mut fixed = Vec::new();
    let index_path = root.join("docs/specs/_index.md");

    // Collect registry fixes to batch-apply
    let mut index_content = if index_path.exists() {
        std::fs::read_to_string(&index_path)?
    } else {
        String::new()
    };
    let mut index_modified = false;

    for diag in &result.diagnostics {
        let Some(fix) = &diag.fix else { continue };
        match fix {
            AutoFix::CreateDirectory { path } => {
                let full = root.join(path);
                std::fs::create_dir_all(&full)?;
                fixed.push(format!("Created directory: {path}"));
            }
            AutoFix::AddRegistryEntry {
                spec_id,
                title,
                status,
                location,
            } => {
                let new_row = format!("| {spec_id} | {title} | {status} | {location} |");
                let section = if status == "Completed" {
                    "## Completed"
                } else {
                    "## Active"
                };
                // Insert after the table header in the correct section
                if let Some(pos) = index_content.find(section) {
                    // Find the end of the header separator line (|---|---|...)
                    let after_section = &index_content[pos..];
                    if let Some(sep_offset) = after_section.find("|---") {
                        let after_sep = &after_section[sep_offset..];
                        if let Some(newline) = after_sep.find('\n') {
                            let insert_pos = pos + sep_offset + newline + 1;
                            index_content.insert_str(insert_pos, &format!("{new_row}\n"));
                            index_modified = true;
                            fixed.push(format!("Added {spec_id} to registry ({status})"));
                        }
                    }
                }
            }
            AutoFix::RemoveRegistryEntry { spec_id } => {
                let lines: Vec<&str> = index_content.lines().collect();
                let new_lines: Vec<&str> = lines
                    .into_iter()
                    .filter(|line| !line.contains(spec_id))
                    .collect();
                index_content = new_lines.join("\n");
                if !index_content.ends_with('\n') {
                    index_content.push('\n');
                }
                index_modified = true;
                fixed.push(format!("Removed {spec_id} from registry"));
            }
            AutoFix::UpdateRegistryEntry {
                spec_id,
                old_line,
                new_line,
            } => {
                if index_content.contains(old_line.as_str()) {
                    index_content = index_content.replace(old_line.as_str(), new_line.as_str());
                    index_modified = true;
                    fixed.push(format!("Updated {spec_id} registry entry"));
                }
            }
            AutoFix::UpdateTemplate { path, content } => {
                let full = root.join(path);
                if let Some(parent) = full.parent() {
                    std::fs::create_dir_all(parent)?;
                }
                std::fs::write(&full, content)?;
                fixed.push(format!("Updated template: {path}"));
            }
        }
    }

    if index_modified {
        std::fs::write(&index_path, &index_content)?;
    }

    Ok(fixed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exit_code_clean() {
        let r = CheckResult::default();
        assert_eq!(r.exit_code(), 0);
    }

    #[test]
    fn exit_code_warning() {
        let mut r = CheckResult::default();
        r.diagnostics.push(Diagnostic {
            severity: Severity::Warning,
            check: "test".into(),
            message: "test warning".into(),
            fix: None,
        });
        assert_eq!(r.exit_code(), 1);
        assert!(!r.has_errors());
        assert!(r.has_warnings());
    }

    #[test]
    fn exit_code_error() {
        let mut r = CheckResult::default();
        r.diagnostics.push(Diagnostic {
            severity: Severity::Error,
            check: "test".into(),
            message: "test error".into(),
            fix: None,
        });
        assert_eq!(r.exit_code(), 2);
        assert!(r.has_errors());
    }

    #[test]
    fn has_fixable() {
        let mut r = CheckResult::default();
        assert!(!r.has_fixable());
        r.diagnostics.push(Diagnostic {
            severity: Severity::Warning,
            check: "test".into(),
            message: "fixable".into(),
            fix: Some(AutoFix::CreateDirectory {
                path: "test".into(),
            }),
        });
        assert!(r.has_fixable());
    }

    #[test]
    fn merge_combines_diagnostics() {
        let mut a = CheckResult::default();
        a.diagnostics.push(Diagnostic {
            severity: Severity::Error,
            check: "a".into(),
            message: "err".into(),
            fix: None,
        });
        let mut b = CheckResult::default();
        b.diagnostics.push(Diagnostic {
            severity: Severity::Warning,
            check: "b".into(),
            message: "warn".into(),
            fix: None,
        });
        a.merge(b);
        assert_eq!(a.diagnostics.len(), 2);
        assert!(a.has_errors());
        assert!(a.has_warnings());
    }
}
