mod interactive;

use anyhow::Result;
use clap::{Parser, Subcommand};
use console::style;
use harn_core::context::WriteStatus;
use harn_core::{HarnConfig, ProjectContext, url_encode};
use harn_modules::ModuleRegistry;
use std::io::IsTerminal;
use std::path::PathBuf;
use std::process::Command;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser)]
#[command(
    name = "harn",
    about = "Universal project harness with SDD methodology",
    version = VERSION,
    after_help = "METHODOLOGY:\n  \
        Spec-Driven Development (SDD) + Harness Engineering.\n  \
        Define features as Specs, implement with AI-assisted workflows,\n  \
        enforce quality gates, maintain SSOT documentation."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new project (interactive)
    Init {
        /// Project directory (default: current directory)
        #[arg(default_value = ".")]
        directory: PathBuf,

        /// Config file to use instead of interactive prompts
        #[arg(short, long)]
        config: Option<PathBuf>,

        /// Overwrite existing files
        #[arg(short, long)]
        force: bool,

        /// Preview what would be created without writing files
        #[arg(long)]
        dry_run: bool,
    },

    /// Add a module to an existing project
    Add {
        /// Module to add (sdd, ci, agent, build, ide, git, docker, env, quality)
        module: String,

        /// Project directory
        #[arg(default_value = ".")]
        directory: PathBuf,

        /// Overwrite existing files
        #[arg(short, long)]
        force: bool,

        /// Preview what would be created without writing files
        #[arg(long)]
        dry_run: bool,
    },

    /// Create a new Spec
    Spec {
        /// Spec title
        title: Option<String>,

        /// Project directory
        #[arg(short, long, default_value = ".")]
        directory: PathBuf,
    },

    /// Diagnose SDD project health
    Doctor {
        /// Project directory
        #[arg(default_value = ".")]
        directory: PathBuf,

        /// Auto-fix safe issues
        #[arg(long)]
        fix: bool,
    },

    /// List available modules
    Modules,

    /// Generate an example harn.toml
    Example {
        /// Output path
        #[arg(default_value = "harn.toml")]
        output: PathBuf,
    },

    /// Submit an issue to the harn project
    Issue {
        /// Issue type: bug, feature, question
        #[arg(long, value_parser = ["bug", "feature", "question"])]
        r#type: Option<String>,

        /// Issue title
        #[arg(long)]
        title: Option<String>,

        /// Issue body / description
        #[arg(long)]
        body: Option<String>,

        /// Open browser to new issue page instead of creating via gh CLI
        #[arg(long)]
        open: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init {
            directory,
            config,
            force,
            dry_run,
        } => cmd_init(directory, config, force, dry_run),
        Commands::Add {
            module,
            directory,
            force,
            dry_run,
        } => cmd_add(&module, directory, force, dry_run),
        Commands::Spec { title, directory } => cmd_spec(title, directory),
        Commands::Doctor { directory, fix } => cmd_doctor(directory, fix),
        Commands::Modules => {
            cmd_modules();
            Ok(())
        }
        Commands::Example { output } => cmd_example(output),
        Commands::Issue {
            r#type,
            title,
            body,
            open,
        } => cmd_issue(r#type, title, body, open),
    }
}

fn cmd_init(
    directory: PathBuf,
    config_path: Option<PathBuf>,
    force: bool,
    dry_run: bool,
) -> Result<()> {
    println!(
        "{} v{VERSION} — Project Harness with SDD",
        style("harn").cyan().bold()
    );
    println!();

    let root = if directory.is_absolute() {
        directory
    } else {
        std::env::current_dir()?.join(&directory)
    };
    std::fs::create_dir_all(&root)?;
    let root = root.canonicalize()?;

    // Load config or run interactive setup
    let config = if let Some(path) = config_path {
        HarnConfig::load(&path)?
    } else {
        interactive::gather_config(&root)?
    };

    // Save config for reproducibility
    if !dry_run {
        config.save(&root.join("harn.toml"))?;
    }

    // Show summary
    println!();
    println!("{}", style("Configuration:").bold());
    println!("  Name:      {}", style(&config.project.name).green());
    println!("  Type:      {}", config.project.r#type);
    println!(
        "  Languages: {}",
        if config.stacks.languages.is_empty() {
            "(none)".into()
        } else {
            config.stacks.languages.join(", ")
        }
    );
    println!("  Modules:   {}", config.enabled_modules().join(", "));
    println!();

    // Execute modules
    let mut ctx = ProjectContext::new(root, config);
    ctx.force = force;
    ctx.dry_run = dry_run;

    if dry_run {
        println!(
            "{}",
            style("Dry-run mode — no files will be written").yellow()
        );
        println!();
    }

    run_enabled_modules(&mut ctx)?;

    if dry_run {
        println!();
        println!(
            "{}",
            style("Dry-run complete. Re-run without --dry-run to apply.").yellow()
        );
        return Ok(());
    }

    println!();
    println!("{}", style("Setup complete!").green().bold());
    println!();
    println!("Next steps:");
    println!("  1. Review and customize {}", style("CLAUDE.md").cyan());
    println!(
        "  2. Edit {} permissions",
        style(".claude/settings.json").cyan()
    );
    println!(
        "  3. Create your first spec: {}",
        style("/spec create \"Feature\"").yellow()
    );
    println!(
        "  4. Start developing: {}",
        style("/implement SPEC-001").yellow()
    );
    println!("  5. Ship changes: {}", style("/ship").yellow());

    Ok(())
}

fn cmd_add(module_id: &str, directory: PathBuf, force: bool, dry_run: bool) -> Result<()> {
    let root = if directory.is_absolute() {
        directory
    } else {
        std::env::current_dir()?.join(&directory)
    };
    let root = root.canonicalize()?;

    // Load existing config or create minimal one
    let config_path = root.join("harn.toml");
    let config = if config_path.exists() {
        HarnConfig::load(&config_path)?
    } else {
        let name = root
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("project")
            .to_string();
        HarnConfig::default_all(name)
    };

    let mut ctx = ProjectContext::new(root, config);
    ctx.force = force;
    ctx.dry_run = dry_run;

    if dry_run {
        println!(
            "{}",
            style("Dry-run mode — no files will be written").yellow()
        );
        println!();
    }

    let registry = ModuleRegistry::new();
    if let Some(module) = registry.get(module_id) {
        println!(
            "Adding {} {}...",
            style(module.name()).cyan(),
            style(format!("({module_id})")).dim()
        );
        let files = module.generate(&mut ctx)?;
        print_file_ops(&files);
    } else {
        eprintln!(
            "{} Unknown module: {}",
            style("Error:").red().bold(),
            module_id
        );
        eprintln!("Available: {}", registry.ids().join(", "));
        std::process::exit(1);
    }

    Ok(())
}

fn cmd_spec(title: Option<String>, directory: PathBuf) -> Result<()> {
    let root = if directory.is_absolute() {
        directory
    } else {
        std::env::current_dir()?.join(&directory)
    };
    let root = root.canonicalize()?;

    let index_path = root.join("docs/specs/_index.md");
    if !index_path.exists() {
        eprintln!(
            "{} No spec registry found. Run `harn add sdd` first.",
            style("Error:").red().bold()
        );
        std::process::exit(1);
    }

    let title = match title {
        Some(t) => t,
        None => dialoguer::Input::new()
            .with_prompt("Spec title")
            .interact_text()?,
    };

    // Find next spec number
    let index_content = std::fs::read_to_string(&index_path)?;
    let last_num = find_last_spec_num(&index_content);
    let next_num = last_num + 1;
    let spec_id = format!("SPEC-{next_num:03}");

    let spec_dir = root.join(format!("docs/specs/active/{spec_id}"));
    std::fs::create_dir_all(&spec_dir)?;

    let today = harn_core::date::today();

    // Generate PRD
    let prd_template = std::fs::read_to_string(root.join("docs/specs/_templates/prd.md"))
        .unwrap_or_else(|_| {
            include_str!("../../../templates/sdd/specs/_templates/prd.md").to_string()
        });
    let prd = prd_template
        .replace("SPEC-NNN", &spec_id)
        .replace("[Title]", &title)
        .replace("YYYY-MM-DD", &today);
    std::fs::write(spec_dir.join("prd.md"), &prd)?;

    // Generate TD
    let td_template =
        std::fs::read_to_string(root.join("docs/specs/_templates/technical-design.md"))
            .unwrap_or_else(|_| {
                include_str!("../../../templates/sdd/specs/_templates/technical-design.md")
                    .to_string()
            });
    let td = td_template
        .replace("SPEC-NNN", &spec_id)
        .replace("[Title]", &title)
        .replace("YYYY-MM-DD", &today);
    std::fs::write(spec_dir.join("technical-design.md"), &td)?;

    // Update registry
    let new_row =
        format!("| {spec_id} | {title} | Draft | [active/{spec_id}/](active/{spec_id}/) |");
    let updated = if index_content.contains("## How to Create") {
        index_content.replace(
            "## How to Create",
            &format!("{new_row}\n\n## How to Create"),
        )
    } else {
        format!("{index_content}\n{new_row}\n")
    };
    std::fs::write(&index_path, updated)?;

    println!(
        "{} {}: {}",
        style("Created").green().bold(),
        style(&spec_id).cyan(),
        title
    );
    println!("  PRD: docs/specs/active/{spec_id}/prd.md");
    println!("  TD:  docs/specs/active/{spec_id}/technical-design.md");

    Ok(())
}

fn cmd_doctor(directory: PathBuf, fix: bool) -> Result<()> {
    let root = if directory.is_absolute() {
        directory
    } else {
        std::env::current_dir()?.join(&directory)
    };
    let root = root.canonicalize()?;

    let mut result = harn_core::doctor::CheckResult::default();
    let has_sdd = root.join("docs/specs").exists();
    let config_path = root.join("harn.toml");
    let config = if config_path.exists() {
        HarnConfig::load(&config_path).ok()
    } else {
        None
    };

    if !has_sdd && config.is_none() {
        eprintln!(
            "{} No harn project found. Run `harn init` first.",
            style("Error:").red().bold()
        );
        std::process::exit(1);
    }

    println!("{} project health...\n", style("Checking").blue().bold());

    // SDD checks (if SDD structure exists)
    if has_sdd {
        println!("{}", style("=== SDD Checks ===").bold().underlined());
        println!();
        result.merge(harn_modules::sdd_checks::run_all_checks(&root));
        println!();
    }

    // Project-wide checks (if harn.toml exists)
    if let Some(cfg) = &config {
        println!("{}", style("=== Module Checks ===").bold().underlined());
        println!();
        result.merge(harn_modules::project_checks::run_all_project_checks(
            &root, cfg,
        ));
        println!();
    }

    if fix && result.has_fixable() {
        println!();
        println!("{}", style("Applying fixes...").blue().bold());
        println!();
        let fixed = harn_core::doctor::apply_fixes(&root, &result)?;
        for f in &fixed {
            println!("  {} {f}", style("fixed").green());
        }

        println!();
        println!("{}", style("Re-checking...").blue().bold());
        println!();

        let mut recheck = harn_core::doctor::CheckResult::default();
        if has_sdd {
            recheck.merge(harn_modules::sdd_checks::run_all_checks(&root));
        }
        if let Some(cfg) = &config {
            recheck.merge(harn_modules::project_checks::run_all_project_checks(
                &root, cfg,
            ));
        }
        harn_core::doctor::print_summary(&recheck);
        std::process::exit(recheck.exit_code());
    }

    harn_core::doctor::print_summary(&result);
    std::process::exit(result.exit_code());
}

fn cmd_modules() {
    let registry = ModuleRegistry::new();
    println!("{}", style("Available modules:").bold());
    println!();
    for module in registry.all() {
        println!(
            "  {:<12} {}",
            style(module.id()).cyan(),
            module.description()
        );
    }
}

fn cmd_example(output: PathBuf) -> Result<()> {
    let config = HarnConfig::default_all("my-project".into());
    config.save(&output)?;
    println!(
        "{} Example config written to {}",
        style("OK").green().bold(),
        output.display()
    );
    Ok(())
}

fn cmd_issue(
    issue_type: Option<String>,
    title: Option<String>,
    body: Option<String>,
    open: bool,
) -> Result<()> {
    println!(
        "{} v{VERSION} — Submit an Issue",
        style("harn").cyan().bold()
    );
    println!();

    // Resolve issue type: flag → interactive
    let type_options = &["Bug Report", "Feature Request", "Question"];
    let type_labels = &["bug", "feature", "question"];
    let type_idx = if let Some(ref t) = issue_type {
        type_labels
            .iter()
            .position(|l| l == t)
            .expect("clap validates value_parser")
    } else {
        dialoguer::FuzzySelect::new()
            .with_prompt("Issue type")
            .items(type_options)
            .default(0)
            .interact()?
    };
    let issue_type_display = type_options[type_idx];

    // Resolve title: flag → interactive
    let title = if let Some(t) = title {
        t
    } else {
        dialoguer::Input::new()
            .with_prompt("Title")
            .interact_text()?
    };

    // Resolve body: flag → stdin → interactive
    let description = if let Some(b) = body {
        b
    } else if !std::io::stdin().is_terminal() {
        let mut buf = String::new();
        std::io::Read::read_to_string(&mut std::io::stdin(), &mut buf)?;
        buf
    } else {
        dialoguer::Input::new()
            .with_prompt("Description (optional)")
            .allow_empty(true)
            .interact_text()?
    };

    let label = match type_idx {
        0 => "bug",
        1 => "enhancement",
        _ => "question",
    };

    // Build body
    let env_info = format!(
        "**harn version:** {VERSION}\n**OS:** {} {}\n**Arch:** {}",
        std::env::consts::OS,
        std::env::consts::FAMILY,
        std::env::consts::ARCH,
    );

    let config_summary = load_config_summary().unwrap_or_default();

    let full_body = format!(
        "## {issue_type_display}\n\n{description}\n\n## Environment\n\n{env_info}{config_summary}"
    );

    // --open: just open browser with pre-filled URL
    if open {
        return open_issue_in_browser(&title, &full_body, label).or_else(|_| {
            let url = format!(
                "https://github.com/wutongshenqiu/harn/issues/new?title={}&body={}&labels={}",
                url_encode(&title),
                url_encode(&full_body),
                url_encode(label),
            );
            println!("Open this URL to submit your issue:\n");
            println!("  {url}");
            Ok(())
        });
    }

    println!();

    // Try gh CLI first
    let gh_result = Command::new("gh")
        .args([
            "issue",
            "create",
            "--repo",
            "wutongshenqiu/harn",
            "--title",
            &title,
            "--body",
            &full_body,
            "--label",
            label,
        ])
        .output();

    match gh_result {
        Ok(output) if output.status.success() => {
            let url = String::from_utf8_lossy(&output.stdout);
            println!(
                "{} Issue created: {}",
                style("OK").green().bold(),
                url.trim()
            );
        }
        _ => {
            // Fallback: open in browser
            if open_issue_in_browser(&title, &full_body, label).is_err() {
                // Final fallback: print URL
                let url = format!(
                    "https://github.com/wutongshenqiu/harn/issues/new?title={}&body={}&labels={}",
                    url_encode(&title),
                    url_encode(&full_body),
                    url_encode(label),
                );
                println!("Open this URL to submit your issue:\n");
                println!("  {url}");
            }
        }
    }

    Ok(())
}

fn load_config_summary() -> Option<String> {
    let config = HarnConfig::load(std::path::Path::new("harn.toml")).ok()?;
    let langs = if config.stacks.languages.is_empty() {
        "none".to_string()
    } else {
        config.stacks.languages.join(", ")
    };
    let modules = config.enabled_modules().join(", ");
    Some(format!("\n**Languages:** {langs}\n**Modules:** {modules}"))
}

fn open_issue_in_browser(title: &str, body: &str, label: &str) -> Result<()> {
    let url = format!(
        "https://github.com/wutongshenqiu/harn/issues/new?title={}&body={}&labels={}",
        url_encode(title),
        url_encode(body),
        url_encode(label),
    );

    let (cmd, args): (&str, &[&str]) = if cfg!(target_os = "macos") {
        ("open", &[&url])
    } else if cfg!(target_os = "windows") {
        ("cmd", &["/C", "start", &url])
    } else {
        ("xdg-open", &[&url])
    };

    let status = Command::new(cmd).args(args).status()?;
    if status.success() {
        println!(
            "{} Opened issue form in browser.",
            style("OK").green().bold()
        );
        Ok(())
    } else {
        anyhow::bail!("failed to open browser")
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn run_enabled_modules(ctx: &mut ProjectContext) -> Result<()> {
    let registry = ModuleRegistry::new();
    let enabled = ctx.config.enabled_modules();

    for module in registry.all() {
        if enabled.contains(&module.id().to_string()) {
            println!(
                "{} {}...",
                style("Generating").blue(),
                style(module.name()).bold()
            );
            let files = module.generate(ctx)?;
            print_file_ops(&files);
        }
    }

    Ok(())
}

fn print_file_ops(files: &[(String, WriteStatus)]) {
    for (path, status) in files {
        let label = match status {
            WriteStatus::Created | WriteStatus::WouldCreate => style("  CREATE").green(),
            WriteStatus::Overwritten => style("   FORCE").magenta(),
            WriteStatus::Skipped => style("    SKIP").dim(),
            WriteStatus::WouldOverwrite => style("   FORCE").yellow(),
        };
        println!("  {label} {path}");
    }
}

fn find_last_spec_num(content: &str) -> u32 {
    let mut max = 0u32;
    for cap in content.split("SPEC-").skip(1) {
        if let Some(num_str) = cap.split(|c: char| !c.is_ascii_digit()).next() {
            if let Ok(n) = num_str.parse::<u32>() {
                if n > max {
                    max = n;
                }
            }
        }
    }
    max
}
