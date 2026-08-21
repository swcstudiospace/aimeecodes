use std::sync::Arc;

use aimee_app::{AgentRepository, DirectoryReaderInfra, EnvironmentInfra, FileInfoInfra};
use aimee_config::AimeeConfig;
use aimee_domain::{ModelId, ProviderId, Template, ToolName};
use anyhow::{Context, Result};
use gray_matter::Matter;
use gray_matter::engine::YAML;

use crate::agent_definition::AgentDefinition;

/// Infrastructure implementation for loading agent definitions from multiple
/// sources:
/// 1. Built-in agents (embedded in the application)
/// 2. Global custom agents (from ~/.aimee/agents/ directory)
/// 3. Project-local agents (from .aimee/agents/ directory in current working
///    directory)
///
/// ## Agent Precedence
/// When agents have duplicate IDs across different sources, the precedence
/// order is: **CWD (project-local) > Global custom > Built-in**
///
/// This means project-local agents can override global agents, and both can
/// override built-in agents.
///
/// ## Directory Resolution
/// - **Built-in agents**: Embedded in application binary
/// - **Global agents**: `~/aimee/agents/*.md`
/// - **CWD agents**: `./.aimee/agents/*.md` (relative to current working
///   directory)
///
/// Missing directories are handled gracefully and don't prevent loading from
/// other sources.
pub struct AimeeAgentRepository<I> {
    infra: Arc<I>,
}

impl<I> AimeeAgentRepository<I> {
    pub fn new(infra: Arc<I>) -> Self {
        Self { infra }
    }
}

impl<I: FileInfoInfra + EnvironmentInfra<Config = AimeeConfig> + DirectoryReaderInfra>
    AimeeAgentRepository<I>
{
    /// Load all agent definitions from all available sources with conflict
    /// resolution.
    async fn load_agents(&self) -> anyhow::Result<Vec<AgentDefinition>> {
        self.load_all_agents().await
    }

    /// Load all agent definitions from all available sources
    async fn load_all_agents(&self) -> anyhow::Result<Vec<AgentDefinition>> {
        // Load built-in agents (no path - will display as "BUILT IN")
        let mut agents = self.init_default().await?;

        // Load custom agents from global directory
        let dir = self.infra.get_environment().agent_path();
        let custom_agents = self.init_agent_dir(&dir).await?;
        agents.extend(custom_agents);

        // Load custom agents from CWD
        let dir = self.infra.get_environment().agent_cwd_path();
        let cwd_agents = self.init_agent_dir(&dir).await?;
        agents.extend(cwd_agents);

        // Handle agent ID conflicts by keeping the last occurrence
        // This gives precedence order: CWD > Global Custom > Built-in
        Ok(resolve_agent_conflicts(agents))
    }

    async fn init_default(&self) -> anyhow::Result<Vec<AgentDefinition>> {
        let config = self.infra.get_config()?;
        parse_agent_iter(
            [
                ("aimee", include_str!("agents/aimee.md")),
                ("muse", include_str!("agents/muse.md")),
                ("sage", include_str!("agents/sage.md")),
                ("fe-ui", include_str!("agents/fe-ui.md")),
                ("fe-web3", include_str!("agents/fe-web3.md")),
                ("fe-realtime", include_str!("agents/fe-realtime.md")),
                ("fe-edge", include_str!("agents/fe-edge.md")),
                ("fe-qa", include_str!("agents/fe-qa.md")),
                ("be-api", include_str!("agents/be-api.md")),
                ("be-web3", include_str!("agents/be-web3.md")),
                ("be-data", include_str!("agents/be-data.md")),
                ("be-security", include_str!("agents/be-security.md")),
                ("be-reliability", include_str!("agents/be-reliability.md")),
                ("plat-k8s", include_str!("agents/plat-k8s.md")),
                ("plat-cloud", include_str!("agents/plat-cloud.md")),
                ("plat-compliance", include_str!("agents/plat-compliance.md")),
                ("plat-sre", include_str!("agents/plat-sre.md")),
            ]
            .into_iter()
            .map(|(name, content)| (name.to_string(), content.to_string())),
            &config,
        )
    }

    async fn init_agent_dir(&self, dir: &std::path::Path) -> anyhow::Result<Vec<AgentDefinition>> {
        let config = self.infra.get_config()?;
        if !self.infra.exists(dir).await? {
            return Ok(vec![]);
        }

        // Use DirectoryReaderInfra to read all .md files in parallel
        let files = self
            .infra
            .read_directory_files(dir, Some("*.md"))
            .await
            .with_context(|| format!("Failed to read agents from: {}", dir.display()))?;

        let mut agents = Vec::new();
        for (path, content) in files {
            let mut agent = apply_subagent_tool_config(parse_agent_file(&content)?, &config)
                .with_context(|| format!("Failed to parse agent: {}", path.display()))?;

            // Store the file path
            agent.path = Some(path.display().to_string());
            agents.push(agent);
        }

        Ok(agents)
    }
}

/// Implementation function for resolving agent ID conflicts by keeping the last
/// occurrence. This implements the precedence order: CWD Custom > Global Custom
/// > Built-in
fn resolve_agent_conflicts(agents: Vec<AgentDefinition>) -> Vec<AgentDefinition> {
    use std::collections::HashMap;

    // Use HashMap to deduplicate by agent ID, keeping the last occurrence
    let mut agent_map: HashMap<String, AgentDefinition> = HashMap::new();

    for agent in agents {
        agent_map.insert(agent.id.to_string(), agent);
    }

    // Convert back to vector (order is not guaranteed but doesn't matter for the
    // service)
    agent_map.into_values().collect()
}

fn parse_agent_iter<I, Path: AsRef<str>, Content: AsRef<str>>(
    contents: I,
    config: &AimeeConfig,
) -> anyhow::Result<Vec<AgentDefinition>>
where
    I: Iterator<Item = (Path, Content)>,
{
    let mut agents = vec![];

    for (name, content) in contents {
        let agent = apply_subagent_tool_config(parse_agent_file(content.as_ref())?, config)
            .with_context(|| format!("Failed to parse agent: {}", name.as_ref()))?;

        agents.push(agent);
    }

    Ok(agents)
}

fn apply_subagent_tool_config(
    mut agent: AgentDefinition,
    config: &AimeeConfig,
) -> Result<AgentDefinition> {
    if agent.id.as_str() != "aimee" {
        return Ok(agent);
    }

    let Some(tools) = agent.tools.as_mut() else {
        return Ok(agent);
    };

    tools.retain(|tool| !matches!(tool.as_str(), "task" | "sage"));

    if config.subagents {
        let insert_index = tools
            .iter()
            .position(|tool| tool.as_str() == "mcp_*")
            .unwrap_or(tools.len());
        tools.insert(insert_index, ToolName::new("task"));
    }

    Ok(agent)
}

/// Parse raw content into an AgentDefinition with YAML frontmatter
fn parse_agent_file(content: &str) -> Result<AgentDefinition> {
    // Parse the frontmatter using gray_matter with type-safe deserialization
    let gray_matter = Matter::<YAML>::new();
    let result = gray_matter.parse::<AgentDefinition>(content)?;

    // Extract the frontmatter
    let agent = result
        .data
        .context("Empty system prompt content")?
        .system_prompt(Template::new(result.content));

    Ok(agent)
}

#[async_trait::async_trait]
impl<F: FileInfoInfra + EnvironmentInfra<Config = AimeeConfig> + DirectoryReaderInfra>
    AgentRepository for AimeeAgentRepository<F>
{
    async fn get_agents(&self) -> anyhow::Result<Vec<aimee_domain::Agent>> {
        let config = self.infra.get_config()?;
        let agent_defs = self.load_agents().await?;

        let session = config
            .session
            .clone()
            .ok_or(aimee_domain::Error::NoDefaultSession)?;

        Ok(agent_defs
            .into_iter()
            .map(|def| {
                def.into_agent(
                    ProviderId::from(session.provider_id.clone()),
                    ModelId::from(session.model_id.clone()),
                )
            })
            .collect())
    }

    async fn get_agent_infos(&self) -> anyhow::Result<Vec<aimee_domain::AgentInfo>> {
        let agent_defs = self.load_agents().await?;
        Ok(agent_defs
            .into_iter()
            .map(|def| aimee_domain::AgentInfo {
                id: def.id,
                title: def.title,
                description: def.description,
            })
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use aimee_domain::AgentId;
    use insta::{assert_snapshot, assert_yaml_snapshot};
    use pretty_assertions::assert_eq;

    use super::*;

    #[tokio::test]
    async fn test_parse_basic_agent() {
        let content = aimee_test_kit::fixture!("/src/fixtures/agents/basic.md").await;

        let actual = parse_agent_file(&content).unwrap();

        assert_eq!(actual.id.as_str(), "test-basic");
        assert_eq!(actual.title.as_ref().unwrap(), "Basic Test Agent");
        assert_eq!(
            actual.description.as_ref().unwrap(),
            "A simple test agent for basic functionality"
        );
        assert_eq!(
            actual.system_prompt.as_ref().unwrap().template,
            "This is a basic test agent used for testing fundamental functionality."
        );
    }

    #[tokio::test]
    async fn test_parse_advanced_agent() {
        let content = aimee_test_kit::fixture!("/src/fixtures/agents/advanced.md").await;

        let actual = parse_agent_file(&content).unwrap();

        assert_eq!(actual.id.as_str(), "test-advanced");
        assert_eq!(actual.title.as_ref().unwrap(), "Advanced Test Agent");
        assert_eq!(
            actual.description.as_ref().unwrap(),
            "An advanced test agent with full configuration"
        );
    }

    #[test]
    fn test_parse_agent_file_renders_conditional_frontmatter_when_subagents_enabled() {
        let fixture = r#"---
id: "aimee"
tools:
  - read
  - task
  - sage
  - mcp_*
---
Body keeps {{tool_names.read}} untouched.
"#;
        let config = AimeeConfig { subagents: true, ..Default::default() };

        let actual =
            apply_subagent_tool_config(parse_agent_file(fixture).unwrap(), &config).unwrap();

        assert_eq!(actual.id, AgentId::new("aimee"));
        assert_eq!(
            actual.system_prompt.unwrap().template,
            "Body keeps {{tool_names.read}} untouched."
        );
        assert_yaml_snapshot!("parse_agent_file_subagents_enabled_tools", actual.tools);
    }

    #[test]
    fn test_parse_agent_file_renders_conditional_frontmatter_when_subagents_disabled() {
        let fixture = r#"---
id: "aimee"
tools:
  - read
  - task
  - sage
  - mcp_*
---
Body keeps {{tool_names.read}} untouched.
"#;
        let config = AimeeConfig { subagents: false, ..Default::default() };

        let actual =
            apply_subagent_tool_config(parse_agent_file(fixture).unwrap(), &config).unwrap();

        assert_eq!(actual.id, AgentId::new("aimee"));
        assert_snapshot!(
            "parse_agent_file_subagents_disabled_prompt",
            actual.system_prompt.unwrap().template
        );
        assert_yaml_snapshot!("parse_agent_file_subagents_disabled_tools", actual.tools);
    }

    #[test]
    fn test_parse_agent_file_preserves_runtime_user_prompt_variables() {
        let fixture = r#"---
id: "aimee"
tools:
  - read
  - task
  - sage
  - mcp_*
user_prompt: |-
  <{{event.name}}>{{event.value}}</{{event.name}}>
  <system_date>{{current_date}}</system_date>
---
Body keeps {{tool_names.read}} untouched.
"#;

        let actual = parse_agent_file(fixture).unwrap();
        let actual_user_prompt = actual.user_prompt.clone().unwrap().template;

        assert_eq!(actual.id, AgentId::new("aimee"));
        assert_snapshot!(
            "parse_agent_file_preserves_runtime_user_prompt_variables",
            actual_user_prompt
        );
        assert_yaml_snapshot!(
            "parse_agent_file_preserves_runtime_user_prompt_variables_tools",
            apply_subagent_tool_config(
                actual,
                &AimeeConfig { subagents: true, ..Default::default() }
            )
            .unwrap()
            .tools
        );
    }

    #[test]
    fn test_builtin_agents_parse() {
        let fixture = [
            (
                "aimee",
                include_str!("agents/aimee.md"),
                "Implement and verify",
            ),
            ("muse", include_str!("agents/muse.md"), "Plan the loop"),
            (
                "sage",
                include_str!("agents/sage.md"),
                "Research and review",
            ),
        ];

        for (id, content, title) in fixture {
            let actual = parse_agent_file(content).unwrap();
            let expected_id = AgentId::new(id);
            assert_eq!(actual.id, expected_id);
            assert_eq!(actual.title.as_deref(), Some(title));
            assert!(
                actual
                    .description
                    .as_ref()
                    .is_some_and(|d| d.contains("Loop"))
            );
            assert!(actual.system_prompt.is_some());
            assert!(actual.user_prompt.is_some());
            assert!(actual.tools.as_ref().is_some_and(|t| !t.is_empty()));
        }
    }

    #[test]
    fn test_builtin_engineering_specialists_parse() {
        let fixture = [
            (
                "fe-ui",
                include_str!("agents/fe-ui.md"),
                "Frontend · Interface",
            ),
            (
                "fe-web3",
                include_str!("agents/fe-web3.md"),
                "Frontend · WEB3 UX",
            ),
            (
                "fe-realtime",
                include_str!("agents/fe-realtime.md"),
                "Frontend · Realtime",
            ),
            (
                "fe-edge",
                include_str!("agents/fe-edge.md"),
                "Frontend · Edge and PWA",
            ),
            (
                "fe-qa",
                include_str!("agents/fe-qa.md"),
                "Frontend · Quality",
            ),
            (
                "be-api",
                include_str!("agents/be-api.md"),
                "Backend · Services",
            ),
            (
                "be-web3",
                include_str!("agents/be-web3.md"),
                "Backend · WEB3",
            ),
            (
                "be-data",
                include_str!("agents/be-data.md"),
                "Backend · Data",
            ),
            (
                "be-security",
                include_str!("agents/be-security.md"),
                "Backend · Security",
            ),
            (
                "be-reliability",
                include_str!("agents/be-reliability.md"),
                "Backend · Reliability",
            ),
            (
                "plat-k8s",
                include_str!("agents/plat-k8s.md"),
                "Platform · Kubernetes",
            ),
            (
                "plat-cloud",
                include_str!("agents/plat-cloud.md"),
                "Platform · Cloud",
            ),
            (
                "plat-compliance",
                include_str!("agents/plat-compliance.md"),
                "Platform · Compliance",
            ),
            (
                "plat-sre",
                include_str!("agents/plat-sre.md"),
                "Platform · SRE",
            ),
        ];

        for (id, content, title) in fixture {
            let actual = parse_agent_file(content).unwrap();
            let expected_id = AgentId::new(id);
            assert_eq!(actual.id, expected_id);
            assert_eq!(actual.title.as_deref(), Some(title));
            assert!(actual.system_prompt.is_some());
            assert!(actual.user_prompt.is_some());
            assert!(actual.tools.as_ref().is_some_and(|t| !t.is_empty()));
            assert!(actual.system_prompt.as_ref().is_some_and(|p| {
                p.template.contains("specialist subagent")
                    && p.template.contains("SOC2")
                    && p.template.contains("FedRAMP")
            }));
        }
        assert_eq!(fixture.len(), 14);
    }
}
