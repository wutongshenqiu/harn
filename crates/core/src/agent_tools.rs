use anyhow::ensure;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentToolArtifact {
    File(&'static str),
    CommandsDir(&'static str),
}

impl AgentToolArtifact {
    pub fn expected_paths(self, commands: &[String]) -> Vec<String> {
        match self {
            Self::File(path) => vec![path.to_string()],
            Self::CommandsDir(dir) => commands
                .iter()
                .map(|command| format!("{dir}/{command}.md"))
                .collect(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AgentTool {
    pub id: &'static str,
    pub display_name: &'static str,
    pub artifacts: &'static [AgentToolArtifact],
    pub doctor_note: Option<&'static str>,
}

const CLAUDE_ARTIFACTS: &[AgentToolArtifact] = &[
    AgentToolArtifact::File(".claude/settings.json"),
    AgentToolArtifact::CommandsDir(".claude/commands"),
];
const CURSOR_ARTIFACTS: &[AgentToolArtifact] = &[AgentToolArtifact::File(".cursor/rules")];
const WINDSURF_ARTIFACTS: &[AgentToolArtifact] = &[AgentToolArtifact::File(".windsurfrules")];
const CLINE_ARTIFACTS: &[AgentToolArtifact] = &[AgentToolArtifact::File(".clinerules")];
const OPENCODE_ARTIFACTS: &[AgentToolArtifact] =
    &[AgentToolArtifact::CommandsDir(".opencode/commands")];
const QODER_ARTIFACTS: &[AgentToolArtifact] = &[AgentToolArtifact::File(".qoder/rules/harn.md")];
const CODEX_ARTIFACTS: &[AgentToolArtifact] = &[];

pub const SUPPORTED_AGENT_TOOLS: &[AgentTool] = &[
    AgentTool {
        id: "claude",
        display_name: "Claude",
        artifacts: CLAUDE_ARTIFACTS,
        doctor_note: None,
    },
    AgentTool {
        id: "cursor",
        display_name: "Cursor",
        artifacts: CURSOR_ARTIFACTS,
        doctor_note: None,
    },
    AgentTool {
        id: "windsurf",
        display_name: "Windsurf",
        artifacts: WINDSURF_ARTIFACTS,
        doctor_note: None,
    },
    AgentTool {
        id: "cline",
        display_name: "Cline",
        artifacts: CLINE_ARTIFACTS,
        doctor_note: None,
    },
    AgentTool {
        id: "opencode",
        display_name: "OpenCode",
        artifacts: OPENCODE_ARTIFACTS,
        doctor_note: None,
    },
    AgentTool {
        id: "qoder",
        display_name: "Qoder",
        artifacts: QODER_ARTIFACTS,
        doctor_note: None,
    },
    AgentTool {
        id: "codex",
        display_name: "Codex",
        artifacts: CODEX_ARTIFACTS,
        doctor_note: Some("AGENTS.md provides Codex repo context"),
    },
];

pub const AGENT_TOOL_IDS: &[&str] = &[
    "claude", "cursor", "windsurf", "cline", "opencode", "qoder", "codex",
];

pub const AGENT_TOOL_LIST: &str = "claude, cursor, windsurf, cline, opencode, qoder, codex";
pub const AGENT_TOOL_NAMES: &str = "Claude, Cursor, Windsurf, Cline, OpenCode, Qoder, Codex";

pub fn agent_tool(id: &str) -> Option<&'static AgentTool> {
    SUPPORTED_AGENT_TOOLS.iter().find(|tool| tool.id == id)
}

pub fn validate_agent_tools(tools: &[String]) -> anyhow::Result<()> {
    let unsupported: Vec<&str> = tools
        .iter()
        .map(String::as_str)
        .filter(|tool| agent_tool(tool).is_none())
        .collect();

    ensure!(
        unsupported.is_empty(),
        "Unsupported agent tool(s): {}. Supported values: {}",
        unsupported.join(", "),
        AGENT_TOOL_LIST
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{AGENT_TOOL_IDS, SUPPORTED_AGENT_TOOLS, agent_tool, validate_agent_tools};

    #[test]
    fn supported_ids_match_tool_table() {
        let ids: Vec<&str> = SUPPORTED_AGENT_TOOLS.iter().map(|tool| tool.id).collect();
        assert_eq!(ids, AGENT_TOOL_IDS);
    }

    #[test]
    fn codex_has_context_note_and_no_artifacts() {
        let codex = agent_tool("codex").expect("codex should be supported");
        assert!(codex.artifacts.is_empty());
        assert_eq!(
            codex.doctor_note,
            Some("AGENTS.md provides Codex repo context")
        );
    }

    #[test]
    fn validate_agent_tools_rejects_unknown_values() {
        let err = validate_agent_tools(&["codex".into(), "unknown".into()])
            .expect_err("unknown tool should fail validation");

        let message = err.to_string();
        assert!(message.contains("unknown"));
        assert!(message.contains("codex"));
    }
}
