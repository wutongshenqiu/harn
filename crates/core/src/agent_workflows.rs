use anyhow::ensure;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AgentWorkflow {
    pub id: &'static str,
    pub slash_command: &'static str,
    pub purpose: &'static str,
    pub skill_title: &'static str,
    pub skill_description: &'static str,
}

pub const SUPPORTED_AGENT_WORKFLOWS: &[AgentWorkflow] = &[
    AgentWorkflow {
        id: "ship",
        slash_command: "/ship [msg]",
        purpose: "Lint + test + commit + push + PR",
        skill_title: "Ship Changes",
        skill_description: "End-to-end commit pipeline. Use when the user wants to lint, test, commit, push, and open a PR.",
    },
    AgentWorkflow {
        id: "implement",
        slash_command: "/implement SPEC-NNN",
        purpose: "Implement a spec",
        skill_title: "Spec Implementer",
        skill_description: "Implement a spec end-to-end. Use when the user wants code changes for a specific SPEC-NNN.",
    },
    AgentWorkflow {
        id: "spec",
        slash_command: "/spec create/list/advance",
        purpose: "Manage spec lifecycle",
        skill_title: "Spec Manager",
        skill_description: "Manage the spec lifecycle. Use when the user wants to create, inspect, or advance specs.",
    },
    AgentWorkflow {
        id: "lint",
        slash_command: "/lint [fix]",
        purpose: "Run linters",
        skill_title: "Lint Runner",
        skill_description: "Run linting and formatting checks. Use when the user wants lint status or auto-fixes.",
    },
    AgentWorkflow {
        id: "test",
        slash_command: "/test [scope]",
        purpose: "Run tests",
        skill_title: "Test Runner",
        skill_description: "Run project tests. Use when the user wants unit, integration, or scoped test execution.",
    },
    AgentWorkflow {
        id: "review",
        slash_command: "/review [PR#]",
        purpose: "Code review",
        skill_title: "PR Reviewer",
        skill_description: "Review a pull request or diff. Use when the user asks for a code review or PR review.",
    },
    AgentWorkflow {
        id: "diagnose",
        slash_command: "/diagnose [error]",
        purpose: "Diagnose issues",
        skill_title: "Issue Diagnoser",
        skill_description: "Diagnose and fix project issues. Use when the user describes a bug, failure, or broken workflow.",
    },
    AgentWorkflow {
        id: "deps",
        slash_command: "/deps [check/update]",
        purpose: "Manage dependencies",
        skill_title: "Dependency Manager",
        skill_description: "Manage dependencies. Use when the user wants to inspect, update, or reconcile dependencies.",
    },
    AgentWorkflow {
        id: "doc-audit",
        slash_command: "/doc-audit",
        purpose: "Audit docs vs code",
        skill_title: "Doc Auditor",
        skill_description: "Audit documentation against code and generated overlays. Use when the user wants drift or consistency checks.",
    },
    AgentWorkflow {
        id: "issues",
        slash_command: "/issues SPEC-NNN",
        purpose: "Generate issues from Spec",
        skill_title: "Issue Generator",
        skill_description: "Generate GitHub issues from a spec. Use when the user wants implementation issues derived from SPEC-NNN.",
    },
    AgentWorkflow {
        id: "retro",
        slash_command: "/retro",
        purpose: "Session retrospective",
        skill_title: "Retro Runner",
        skill_description: "Run a retrospective on recent work. Use when the user wants workflow improvements or follow-up actions.",
    },
    AgentWorkflow {
        id: "ci",
        slash_command: "/ci [PR#]",
        purpose: "Check CI status",
        skill_title: "CI Inspector",
        skill_description: "Inspect CI status. Use when the user wants workflow runs, PR checks, or failing job diagnosis.",
    },
    AgentWorkflow {
        id: "pr",
        slash_command: "/pr [title]",
        purpose: "Create pull request",
        skill_title: "PR Creator",
        skill_description: "Create or prepare a pull request. Use when the user wants a PR opened or summarized.",
    },
    AgentWorkflow {
        id: "deploy",
        slash_command: "/deploy",
        purpose: "Deploy",
        skill_title: "Deploy Runner",
        skill_description: "Run deployment workflow steps. Use when the user wants to prepare, verify, or execute a deployment.",
    },
    AgentWorkflow {
        id: "sync-commands",
        slash_command: "/sync-commands",
        purpose: "Sync slash commands",
        skill_title: "Workflow Sync",
        skill_description: "Sync agent workflow overlays from the neutral source files. Use when the user wants multi-tool command alignment.",
    },
    AgentWorkflow {
        id: "run-plan",
        slash_command: "/run-plan",
        purpose: "Orchestrate multi-spec long-running plans",
        skill_title: "Plan Orchestrator",
        skill_description: "Orchestrate long-running multi-spec plans. Use when the user wants cross-spec planning or resumable execution.",
    },
];

pub const AGENT_WORKFLOW_IDS: &[&str] = &[
    "ship",
    "implement",
    "spec",
    "lint",
    "test",
    "review",
    "diagnose",
    "deps",
    "doc-audit",
    "issues",
    "retro",
    "ci",
    "pr",
    "deploy",
    "sync-commands",
    "run-plan",
];

pub const DEFAULT_AGENT_WORKFLOW_IDS: &[&str] = &[
    "ship",
    "implement",
    "spec",
    "lint",
    "test",
    "review",
    "diagnose",
    "deps",
    "issues",
    "doc-audit",
    "retro",
    "sync-commands",
    "ci",
    "pr",
    "deploy",
];

pub const AGENT_WORKFLOW_LIST: &str = "ship, implement, spec, lint, test, review, diagnose, deps, doc-audit, issues, retro, ci, pr, deploy, sync-commands, run-plan";

pub fn agent_workflow(id: &str) -> Option<&'static AgentWorkflow> {
    SUPPORTED_AGENT_WORKFLOWS
        .iter()
        .find(|workflow| workflow.id == id)
}

pub fn validate_agent_workflows(commands: &[String]) -> anyhow::Result<()> {
    let unsupported: Vec<&str> = commands
        .iter()
        .map(String::as_str)
        .filter(|command| agent_workflow(command).is_none())
        .collect();

    ensure!(
        unsupported.is_empty(),
        "Unsupported agent workflow(s): {}. Supported values: {}",
        unsupported.join(", "),
        AGENT_WORKFLOW_LIST
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{
        AGENT_WORKFLOW_IDS, DEFAULT_AGENT_WORKFLOW_IDS, SUPPORTED_AGENT_WORKFLOWS, agent_workflow,
        validate_agent_workflows,
    };

    #[test]
    fn supported_ids_match_workflow_table() {
        let ids: Vec<&str> = SUPPORTED_AGENT_WORKFLOWS
            .iter()
            .map(|workflow| workflow.id)
            .collect();
        assert_eq!(ids, AGENT_WORKFLOW_IDS);
    }

    #[test]
    fn default_workflows_are_supported() {
        for workflow_id in DEFAULT_AGENT_WORKFLOW_IDS {
            assert!(
                agent_workflow(workflow_id).is_some(),
                "default workflow should be supported: {workflow_id}"
            );
        }
    }

    #[test]
    fn validate_agent_workflows_rejects_unknown_values() {
        let err = validate_agent_workflows(&["review".into(), "unknown".into()])
            .expect_err("unknown workflow should fail validation");

        let message = err.to_string();
        assert!(message.contains("unknown"));
        assert!(message.contains("review"));
    }
}
