use harn_core::doctor::{AutoFix, CheckResult, Diagnostic, Severity};
use harn_templates::TemplateEngine;
use std::collections::HashMap;
use std::path::Path;

// Required sections in PRD files
const PRD_REQUIRED_SECTIONS: &[&str] = &["## Problem Statement", "## Goals", "## User Stories"];

// Required sections in Technical Design files
const TD_REQUIRED_SECTIONS: &[&str] = &["## Overview", "## Task Breakdown", "## Test Strategy"];

// Template files that ship with harn's SDD module
const SDD_TEMPLATE_FILES: &[&str] = &[
    "sdd/specs/_templates/prd.md",
    "sdd/specs/_templates/technical-design.md",
    "sdd/specs/_templates/research.md",
];

/// Check 1: Directory structure completeness.
pub fn check_directory_structure(root: &Path) -> CheckResult {
    let mut result = CheckResult::default();
    let check = "directory-structure";

    let required = [
        "docs/specs/_index.md",
        "docs/specs/_templates",
        "docs/specs/active",
        "docs/specs/completed",
    ];

    for path in &required {
        let full = root.join(path);
        if full.exists() {
            CheckResult::ok(check, format!("{path} exists"));
        } else {
            let is_dir = !path.contains('.');
            result.push(Diagnostic {
                severity: if *path == "docs/specs/_index.md" {
                    Severity::Error
                } else {
                    Severity::Warning
                },
                check: check.into(),
                message: format!("{path} missing"),
                fix: if is_dir {
                    Some(AutoFix::CreateDirectory {
                        path: (*path).into(),
                    })
                } else {
                    None
                },
            });
        }
    }

    // Check templates dir has at least prd.md
    let templates_dir = root.join("docs/specs/_templates");
    if templates_dir.is_dir() {
        let prd = templates_dir.join("prd.md");
        if prd.exists() {
            let count = std::fs::read_dir(&templates_dir)
                .map(|rd| {
                    rd.filter(|e| {
                        e.as_ref()
                            .is_ok_and(|e| e.path().extension().is_some_and(|ext| ext == "md"))
                    })
                    .count()
                })
                .unwrap_or(0);
            CheckResult::ok(check, format!("_templates/ contains {count} template(s)"));
        } else {
            result.push(Diagnostic {
                severity: Severity::Warning,
                check: check.into(),
                message: "_templates/prd.md missing".into(),
                fix: TemplateEngine::get_embedded_content("sdd/specs/_templates/prd.md").map(
                    |content| AutoFix::UpdateTemplate {
                        path: "docs/specs/_templates/prd.md".into(),
                        content: content.to_vec(),
                    },
                ),
            });
        }
    }

    result
}

/// A parsed registry entry from `_index.md`.
#[derive(Debug, Clone)]
struct RegistryEntry {
    spec_id: String,
    title: String,
    status: String,
    line: String,
}

/// Parse registry entries from `_index.md` content.
fn parse_registry(content: &str) -> Vec<RegistryEntry> {
    let mut entries = Vec::new();
    for line in content.lines() {
        let trimmed = line.trim();
        if !trimmed.starts_with("| SPEC-") {
            continue;
        }
        let cols: Vec<&str> = trimmed
            .split('|')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .collect();
        if cols.len() >= 4 {
            entries.push(RegistryEntry {
                spec_id: cols[0].to_string(),
                title: cols[1].to_string(),
                status: cols[2].to_string(),
                line: line.to_string(),
            });
        }
    }
    entries
}

/// Scan filesystem for spec directories under active/ and completed/.
fn scan_spec_dirs(root: &Path) -> HashMap<String, String> {
    let mut found = HashMap::new(); // spec_id -> "active" or "completed"
    for (label, subdir) in [
        ("active", "docs/specs/active"),
        ("completed", "docs/specs/completed"),
    ] {
        let dir = root.join(subdir);
        if let Ok(rd) = std::fs::read_dir(&dir) {
            for entry in rd.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                if name.starts_with("SPEC-") && entry.path().is_dir() {
                    found.insert(name, label.to_string());
                }
            }
        }
    }
    found
}

/// Check 2: Registry ↔ filesystem consistency.
pub fn check_registry_consistency(root: &Path) -> CheckResult {
    let mut result = CheckResult::default();
    let check = "registry-consistency";

    let index_path = root.join("docs/specs/_index.md");
    let Ok(content) = std::fs::read_to_string(&index_path) else {
        result.push(Diagnostic {
            severity: Severity::Error,
            check: check.into(),
            message: "Cannot read docs/specs/_index.md".into(),
            fix: None,
        });
        return result;
    };

    let entries = parse_registry(&content);
    let fs_specs = scan_spec_dirs(root);

    // Check each registry entry has a matching directory
    let mut registered_ids: HashMap<String, &RegistryEntry> = HashMap::new();
    for entry in &entries {
        registered_ids.insert(entry.spec_id.clone(), entry);

        let expected_dir_label = if entry.status == "Completed" {
            "completed"
        } else {
            "active"
        };

        match fs_specs.get(&entry.spec_id) {
            Some(actual_label) => {
                if actual_label == expected_dir_label {
                    CheckResult::ok(
                        check,
                        format!("{} registry entry consistent", entry.spec_id),
                    );
                } else {
                    // Directory exists but in wrong location
                    let new_location = format!("{actual_label}/{}/", entry.spec_id);
                    let new_status = if actual_label == "completed" {
                        "Completed"
                    } else {
                        &entry.status
                    };
                    let new_line = format!(
                        "| {} | {} | {} | {} |",
                        entry.spec_id, entry.title, new_status, new_location
                    );
                    result.push(Diagnostic {
                        severity: Severity::Warning,
                        check: check.into(),
                        message: format!(
                            "{} listed as {} but directory is in {actual_label}/",
                            entry.spec_id, entry.status
                        ),
                        fix: Some(AutoFix::UpdateRegistryEntry {
                            spec_id: entry.spec_id.clone(),
                            old_line: entry.line.clone(),
                            new_line,
                        }),
                    });
                }
            }
            None => {
                result.push(Diagnostic {
                    severity: Severity::Warning,
                    check: check.into(),
                    message: format!("{} in registry but directory not found", entry.spec_id),
                    fix: Some(AutoFix::RemoveRegistryEntry {
                        spec_id: entry.spec_id.clone(),
                    }),
                });
            }
        }
    }

    // Check each filesystem spec has a registry entry
    for (spec_id, label) in &fs_specs {
        if !registered_ids.contains_key(spec_id) {
            let title = read_spec_title(root, spec_id, label);
            let status = if label == "completed" {
                "Completed"
            } else {
                "Active"
            };
            result.push(Diagnostic {
                severity: Severity::Error,
                check: check.into(),
                message: format!(
                    "{spec_id} directory exists in {label}/ but has no registry entry"
                ),
                fix: Some(AutoFix::AddRegistryEntry {
                    spec_id: spec_id.clone(),
                    title,
                    status: status.into(),
                    location: format!("{label}/{spec_id}/"),
                }),
            });
        }
    }

    result
}

/// Try to extract the spec title from its PRD file.
fn read_spec_title(root: &Path, spec_id: &str, dir_label: &str) -> String {
    let prd_path = root.join(format!("docs/specs/{dir_label}/{spec_id}/prd.md"));
    if let Ok(content) = std::fs::read_to_string(&prd_path) {
        // Look for the first H1 heading
        for line in content.lines() {
            if let Some(title) = line.strip_prefix("# ") {
                let title = title.trim();
                // Strip "PRD: " prefix if present
                return title.strip_prefix("PRD: ").unwrap_or(title).to_string();
            }
        }
    }
    "Untitled".to_string()
}

/// Check 3: Spec file format completeness.
pub fn check_spec_format(root: &Path) -> CheckResult {
    let mut result = CheckResult::default();
    let check = "spec-format";

    let fs_specs = scan_spec_dirs(root);
    let mut total = 0usize;
    let mut passed = 0usize;

    for (spec_id, label) in &fs_specs {
        let spec_dir = root.join(format!("docs/specs/{label}/{spec_id}"));

        // Check prd.md exists
        let prd_path = spec_dir.join("prd.md");
        if !prd_path.exists() {
            result.push(Diagnostic {
                severity: Severity::Warning,
                check: check.into(),
                message: format!("{spec_id}/prd.md missing"),
                fix: None,
            });
            continue;
        }

        // Check PRD required sections
        total += 1;
        let prd_ok = check_file_sections(
            &prd_path,
            PRD_REQUIRED_SECTIONS,
            spec_id,
            "prd.md",
            check,
            &mut result,
        );
        if prd_ok {
            passed += 1;
        }

        // Check TD if it exists
        let td_path = spec_dir.join("technical-design.md");
        if td_path.exists() {
            total += 1;
            let td_ok = check_file_sections(
                &td_path,
                TD_REQUIRED_SECTIONS,
                spec_id,
                "technical-design.md",
                check,
                &mut result,
            );
            if td_ok {
                passed += 1;
            }
        }
    }

    if total > 0 {
        CheckResult::ok(
            check,
            format!("{passed}/{total} spec files pass format check"),
        );
    }

    result
}

/// Check that a file contains all required section headings.
/// Returns true if all sections are present.
fn check_file_sections(
    path: &Path,
    sections: &[&str],
    spec_id: &str,
    filename: &str,
    check: &str,
    result: &mut CheckResult,
) -> bool {
    let Ok(content) = std::fs::read_to_string(path) else {
        return false;
    };

    let mut all_present = true;
    for section in sections {
        if !content.contains(section) {
            all_present = false;
            result.push(Diagnostic {
                severity: Severity::Warning,
                check: check.into(),
                message: format!("{spec_id}/{filename} missing section: \"{section}\""),
                fix: None,
            });
        }
    }
    all_present
}

/// Check 4: Template drift detection.
pub fn check_template_drift(root: &Path) -> CheckResult {
    let mut result = CheckResult::default();
    let check = "template-drift";

    for embedded_path in SDD_TEMPLATE_FILES {
        let Some(embedded_content) = TemplateEngine::get_embedded_content(embedded_path) else {
            continue;
        };

        // Map embedded path to project path: "sdd/specs/_templates/X" -> "docs/specs/_templates/X"
        let project_rel = embedded_path.replacen("sdd/", "docs/", 1);
        let project_path = root.join(&project_rel);

        if !project_path.exists() {
            result.push(Diagnostic {
                severity: Severity::Warning,
                check: check.into(),
                message: format!("{project_rel} not found (expected from SDD module)"),
                fix: Some(AutoFix::UpdateTemplate {
                    path: project_rel,
                    content: embedded_content.to_vec(),
                }),
            });
            continue;
        }

        let Ok(local_content) = std::fs::read(&project_path) else {
            continue;
        };

        let filename = project_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown");

        if local_content == embedded_content {
            CheckResult::ok(check, format!("_templates/{filename} matches built-in"));
        } else {
            result.push(Diagnostic {
                severity: Severity::Warning,
                check: check.into(),
                message: format!("_templates/{filename} differs from harn built-in"),
                fix: Some(AutoFix::UpdateTemplate {
                    path: project_rel,
                    content: embedded_content.to_vec(),
                }),
            });
        }
    }

    result
}

/// Check 5: Configuration consistency.
pub fn check_config_consistency(root: &Path) -> CheckResult {
    let mut result = CheckResult::default();
    let check = "config-consistency";

    let config_path = root.join("harn.toml");
    if !config_path.exists() {
        result.push(Diagnostic {
            severity: Severity::Warning,
            check: check.into(),
            message: "harn.toml not found".into(),
            fix: None,
        });
        return result;
    }

    let Ok(config) = harn_core::HarnConfig::load(&config_path) else {
        result.push(Diagnostic {
            severity: Severity::Error,
            check: check.into(),
            message: "harn.toml parse error".into(),
            fix: None,
        });
        return result;
    };

    // Check SDD module is enabled (since docs/specs/ exists)
    if config.modules.sdd.is_none() {
        result.push(Diagnostic {
            severity: Severity::Warning,
            check: check.into(),
            message: "SDD structure exists but sdd module not enabled in harn.toml".into(),
            fix: None,
        });
    } else {
        CheckResult::ok(check, "harn.toml valid and SDD module enabled");
    }

    // Check reference docs consistency
    if let Some(sdd) = &config.modules.sdd {
        if sdd.reference {
            let ref_dir = root.join("docs/reference");
            if ref_dir.is_dir() {
                CheckResult::ok(check, "reference docs present (reference=true)");
            } else {
                result.push(Diagnostic {
                    severity: Severity::Warning,
                    check: check.into(),
                    message: "reference=true in config but docs/reference/ missing".into(),
                    fix: Some(AutoFix::CreateDirectory {
                        path: "docs/reference".into(),
                    }),
                });
            }
        }

        if sdd.playbooks {
            let pb_dir = root.join("docs/playbooks");
            if pb_dir.is_dir() {
                CheckResult::ok(check, "playbooks present (playbooks=true)");
            } else {
                result.push(Diagnostic {
                    severity: Severity::Warning,
                    check: check.into(),
                    message: "playbooks=true in config but docs/playbooks/ missing".into(),
                    fix: Some(AutoFix::CreateDirectory {
                        path: "docs/playbooks".into(),
                    }),
                });
            }
        }
    }

    result
}

// Playbook files that should exist in both templates/sdd/playbooks/ and docs/playbooks/
const SDD_PLAYBOOK_FILES: &[&str] = &[
    "sdd/playbooks/create-new-spec.md",
    "sdd/playbooks/coding-agent-workflow.md",
    "sdd/playbooks/write-prd-td.md",
    "sdd/playbooks/add-new-language.md",
];

/// Check 6: Playbook sync between templates/ (embedded) and docs/.
pub fn check_playbook_sync(root: &Path) -> CheckResult {
    let mut result = CheckResult::default();
    let check = "playbook-sync";

    let playbooks_dir = root.join("docs/playbooks");
    if !playbooks_dir.is_dir() {
        return result;
    }

    // Check that every embedded playbook has a corresponding file in docs/playbooks/
    for embedded_path in SDD_PLAYBOOK_FILES {
        let filename = Path::new(embedded_path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_default();

        let project_path = playbooks_dir.join(filename);
        let Some(embedded_content) = TemplateEngine::get_embedded_content(embedded_path) else {
            continue;
        };

        if project_path.exists() {
            CheckResult::ok(check, format!("playbooks/{filename} present"));
        } else {
            result.push(Diagnostic {
                severity: Severity::Warning,
                check: check.into(),
                message: format!("playbooks/{filename} missing (exists in built-in templates)"),
                fix: Some(AutoFix::UpdateTemplate {
                    path: format!("docs/playbooks/{filename}"),
                    content: embedded_content.to_vec(),
                }),
            });
        }
    }

    // Check for docs/playbooks/ files that don't exist in embedded templates
    if let Ok(rd) = std::fs::read_dir(&playbooks_dir) {
        let embedded_filenames: Vec<&str> = SDD_PLAYBOOK_FILES
            .iter()
            .filter_map(|p| Path::new(p).file_name().and_then(|n| n.to_str()))
            .collect();

        for entry in rd.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            let is_md = Path::new(&name)
                .extension()
                .is_some_and(|ext| ext.eq_ignore_ascii_case("md"));
            if is_md && !embedded_filenames.contains(&name.as_str()) {
                result.push(Diagnostic {
                    severity: Severity::Warning,
                    check: check.into(),
                    message: format!(
                        "playbooks/{name} exists locally but not in built-in templates"
                    ),
                    fix: None,
                });
            }
        }
    }

    result
}

/// Check 7: CLAUDE.md consistency with .claude/commands/.
pub fn check_claude_md_consistency(root: &Path) -> CheckResult {
    let mut result = CheckResult::default();
    let check = "claude-md-consistency";

    let claude_md_path = root.join("CLAUDE.md");
    let Ok(claude_md) = std::fs::read_to_string(&claude_md_path) else {
        // No CLAUDE.md — nothing to check
        return result;
    };

    let commands_dir = root.join(".claude/commands");
    if !commands_dir.is_dir() {
        return result;
    }

    // Collect command names from .claude/commands/
    let mut fs_commands: Vec<String> = Vec::new();
    if let Ok(rd) = std::fs::read_dir(&commands_dir) {
        for entry in rd.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if let Some(cmd) = name.strip_suffix(".md") {
                fs_commands.push(cmd.to_string());
            }
        }
    }
    fs_commands.sort();

    // Check each command file has a corresponding entry in CLAUDE.md slash commands table
    for cmd in &fs_commands {
        let pattern = format!("/{cmd}");
        if claude_md.contains(&pattern) {
            CheckResult::ok(check, format!("/{cmd} documented in CLAUDE.md"));
        } else {
            result.push(Diagnostic {
                severity: Severity::Warning,
                check: check.into(),
                message: format!(
                    "/{cmd} exists in .claude/commands/ but not found in CLAUDE.md slash commands table"
                ),
                fix: None,
            });
        }
    }

    result
}

/// Run all SDD health checks.
pub fn run_all_checks(root: &Path) -> CheckResult {
    let mut result = CheckResult::default();

    println!("{}", console::style("[Directory Structure]").bold());
    result.merge(check_directory_structure(root));
    println!();

    println!("{}", console::style("[Registry Consistency]").bold());
    result.merge(check_registry_consistency(root));
    println!();

    println!("{}", console::style("[Spec Format]").bold());
    result.merge(check_spec_format(root));
    println!();

    println!("{}", console::style("[Template Drift]").bold());
    result.merge(check_template_drift(root));
    println!();

    println!("{}", console::style("[Config Consistency]").bold());
    result.merge(check_config_consistency(root));
    println!();

    println!("{}", console::style("[Playbook Sync]").bold());
    result.merge(check_playbook_sync(root));
    println!();

    println!("{}", console::style("[CLAUDE.md Consistency]").bold());
    result.merge(check_claude_md_consistency(root));

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn setup_sdd_project(dir: &Path) {
        // Minimal SDD structure
        fs::create_dir_all(dir.join("docs/specs/active")).unwrap();
        fs::create_dir_all(dir.join("docs/specs/completed")).unwrap();
        fs::create_dir_all(dir.join("docs/specs/_templates")).unwrap();

        // Write templates matching embedded versions
        for tmpl_path in SDD_TEMPLATE_FILES {
            if let Some(content) = TemplateEngine::get_embedded_content(tmpl_path) {
                let project_rel = tmpl_path.replacen("sdd/", "docs/", 1);
                let full = dir.join(&project_rel);
                fs::create_dir_all(full.parent().unwrap()).unwrap();
                fs::write(&full, content).unwrap();
            }
        }

        // Write a basic _index.md
        fs::write(
            dir.join("docs/specs/_index.md"),
            "# Spec Registry\n\n\
             ## Completed\n\n\
             | ID       | Title   | Status    | Location          |\n\
             |----------|---------|-----------|-------------------|\n\n\
             ## Active\n\n\
             | ID       | Title   | Status    | Location          |\n\
             |----------|---------|-----------|-------------------|\n\n\
             ## How to Create a New Spec\n",
        )
        .unwrap();

        // Write harn.toml
        fs::write(
            dir.join("harn.toml"),
            "[project]\nname = \"test-project\"\n\n[modules.sdd]\nplaybooks = false\nreference = false\n",
        )
        .unwrap();
    }

    fn setup_spec(dir: &Path, spec_id: &str, label: &str) {
        let spec_dir = dir.join(format!("docs/specs/{label}/{spec_id}"));
        fs::create_dir_all(&spec_dir).unwrap();
        fs::write(
            spec_dir.join("prd.md"),
            format!(
                "# PRD: Test Feature\n\n\
                 | Field | Value |\n|---|---|\n| Spec ID | {spec_id} |\n\n\
                 ## Problem Statement\nSome problem.\n\n\
                 ## Goals\n- Goal 1\n\n\
                 ## User Stories\n- Story 1\n"
            ),
        )
        .unwrap();
        fs::write(
            spec_dir.join("technical-design.md"),
            format!(
                "# TD: Test Feature\n\n\
                 | Field | Value |\n|---|---|\n| Spec ID | {spec_id} |\n\n\
                 ## Overview\nOverview text.\n\n\
                 ## Task Breakdown\n- [ ] Task 1\n\n\
                 ## Test Strategy\n- Unit tests\n"
            ),
        )
        .unwrap();
    }

    #[test]
    fn directory_structure_all_present() {
        let tmp = tempfile::tempdir().unwrap();
        setup_sdd_project(tmp.path());
        let r = check_directory_structure(tmp.path());
        assert_eq!(r.exit_code(), 0);
    }

    #[test]
    fn directory_structure_missing_dirs() {
        let tmp = tempfile::tempdir().unwrap();
        fs::create_dir_all(tmp.path().join("docs/specs/_templates")).unwrap();
        fs::write(tmp.path().join("docs/specs/_index.md"), "# Spec Registry\n").unwrap();
        // active/ and completed/ missing
        let r = check_directory_structure(tmp.path());
        assert!(r.has_warnings() || r.has_errors());
    }

    #[test]
    fn registry_consistency_clean() {
        let tmp = tempfile::tempdir().unwrap();
        setup_sdd_project(tmp.path());
        setup_spec(tmp.path(), "SPEC-001", "active");

        // Add to registry
        let index = fs::read_to_string(tmp.path().join("docs/specs/_index.md")).unwrap();
        let updated = index.replace(
            "## How to Create",
            "| SPEC-001 | Test Feature | Active | active/SPEC-001/ |\n\n## How to Create",
        );
        fs::write(tmp.path().join("docs/specs/_index.md"), updated).unwrap();

        let r = check_registry_consistency(tmp.path());
        assert!(!r.has_errors());
    }

    #[test]
    fn registry_consistency_orphan_spec() {
        let tmp = tempfile::tempdir().unwrap();
        setup_sdd_project(tmp.path());
        setup_spec(tmp.path(), "SPEC-001", "active");
        // No registry entry for SPEC-001
        let r = check_registry_consistency(tmp.path());
        assert!(r.has_errors());
        assert!(r.diagnostics[0].fix.is_some());
    }

    #[test]
    fn registry_consistency_missing_dir() {
        let tmp = tempfile::tempdir().unwrap();
        setup_sdd_project(tmp.path());

        // Registry has SPEC-001 but no directory
        let index = fs::read_to_string(tmp.path().join("docs/specs/_index.md")).unwrap();
        let updated = index.replace(
            "## How to Create",
            "| SPEC-001 | Ghost Spec | Active | active/SPEC-001/ |\n\n## How to Create",
        );
        fs::write(tmp.path().join("docs/specs/_index.md"), updated).unwrap();

        let r = check_registry_consistency(tmp.path());
        assert!(r.has_warnings());
    }

    #[test]
    fn spec_format_complete() {
        let tmp = tempfile::tempdir().unwrap();
        setup_sdd_project(tmp.path());
        setup_spec(tmp.path(), "SPEC-001", "active");
        let r = check_spec_format(tmp.path());
        assert_eq!(r.exit_code(), 0);
    }

    #[test]
    fn spec_format_missing_section() {
        let tmp = tempfile::tempdir().unwrap();
        setup_sdd_project(tmp.path());
        let spec_dir = tmp.path().join("docs/specs/active/SPEC-001");
        fs::create_dir_all(&spec_dir).unwrap();
        fs::write(
            spec_dir.join("prd.md"),
            "# PRD: Incomplete\n\n## Problem Statement\nSomething.\n",
        )
        .unwrap();
        let r = check_spec_format(tmp.path());
        assert!(r.has_warnings());
    }

    #[test]
    fn template_drift_clean() {
        let tmp = tempfile::tempdir().unwrap();
        setup_sdd_project(tmp.path());
        let r = check_template_drift(tmp.path());
        assert_eq!(r.exit_code(), 0);
    }

    #[test]
    fn template_drift_modified() {
        let tmp = tempfile::tempdir().unwrap();
        setup_sdd_project(tmp.path());
        // Modify a template
        fs::write(
            tmp.path().join("docs/specs/_templates/prd.md"),
            "# Modified PRD template\n",
        )
        .unwrap();
        let r = check_template_drift(tmp.path());
        assert!(r.has_warnings());
        assert!(r.diagnostics[0].fix.is_some());
    }

    #[test]
    fn config_consistency_valid() {
        let tmp = tempfile::tempdir().unwrap();
        setup_sdd_project(tmp.path());
        let r = check_config_consistency(tmp.path());
        assert_eq!(r.exit_code(), 0);
    }

    #[test]
    fn playbook_sync_all_present() {
        let tmp = tempfile::tempdir().unwrap();
        setup_sdd_project(tmp.path());
        fs::create_dir_all(tmp.path().join("docs/playbooks")).unwrap();
        for path in SDD_PLAYBOOK_FILES {
            if let Some(content) = TemplateEngine::get_embedded_content(path) {
                let filename = Path::new(path).file_name().unwrap();
                fs::write(tmp.path().join("docs/playbooks").join(filename), content).unwrap();
            }
        }
        let r = check_playbook_sync(tmp.path());
        assert_eq!(r.exit_code(), 0);
    }

    #[test]
    fn playbook_sync_missing_file() {
        let tmp = tempfile::tempdir().unwrap();
        setup_sdd_project(tmp.path());
        fs::create_dir_all(tmp.path().join("docs/playbooks")).unwrap();
        // Only write one playbook — others should be flagged
        if let Some(content) =
            TemplateEngine::get_embedded_content("sdd/playbooks/create-new-spec.md")
        {
            fs::write(
                tmp.path().join("docs/playbooks/create-new-spec.md"),
                content,
            )
            .unwrap();
        }
        let r = check_playbook_sync(tmp.path());
        assert!(r.has_warnings());
    }

    #[test]
    fn claude_md_consistency_all_documented() {
        let tmp = tempfile::tempdir().unwrap();
        let commands_dir = tmp.path().join(".claude/commands");
        fs::create_dir_all(&commands_dir).unwrap();
        fs::write(commands_dir.join("ship.md"), "ship command").unwrap();
        fs::write(commands_dir.join("test.md"), "test command").unwrap();
        fs::write(
            tmp.path().join("CLAUDE.md"),
            "# Project\n\n| `/ship` | Ship it |\n| `/test` | Test it |\n",
        )
        .unwrap();
        let r = check_claude_md_consistency(tmp.path());
        assert_eq!(r.exit_code(), 0);
    }

    #[test]
    fn claude_md_consistency_missing_command() {
        let tmp = tempfile::tempdir().unwrap();
        let commands_dir = tmp.path().join(".claude/commands");
        fs::create_dir_all(&commands_dir).unwrap();
        fs::write(commands_dir.join("ship.md"), "ship command").unwrap();
        fs::write(commands_dir.join("secret.md"), "secret command").unwrap();
        fs::write(
            tmp.path().join("CLAUDE.md"),
            "# Project\n\n| `/ship` | Ship it |\n",
        )
        .unwrap();
        let r = check_claude_md_consistency(tmp.path());
        assert!(r.has_warnings());
        assert!(r.diagnostics[0].message.contains("secret"));
    }

    #[test]
    fn config_consistency_missing_reference_dir() {
        let tmp = tempfile::tempdir().unwrap();
        setup_sdd_project(tmp.path());
        fs::write(
            tmp.path().join("harn.toml"),
            "[project]\nname = \"test\"\n\n[modules.sdd]\nplaybooks = false\nreference = true\n",
        )
        .unwrap();
        let r = check_config_consistency(tmp.path());
        assert!(r.has_warnings());
    }
}
