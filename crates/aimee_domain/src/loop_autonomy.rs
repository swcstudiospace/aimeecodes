//! Loop autonomy: HITL probes, XML prompt upgrade, telemetry, quality gates.
//!
//! Five probing questions are mandatory before a `/goal` loop becomes active.
//! Tool-failure limits default to unlimited so a turn can keep working.

use serde::{Deserialize, Serialize};

use crate::GoalState;

/// Mandatory number of human probes after a goal is written.
pub const GOAL_PROBE_COUNT: usize = 5;

/// Default tool-failure budget when config omits `max_tool_failure_per_turn`.
/// `usize::MAX` means the orchestrator never force-completes on tool errors.
pub fn unlimited_tool_failures() -> usize {
    usize::MAX
}

/// Canonical HITL questions asked after every `/goal`.
pub fn canonical_probe_questions() -> [&'static str; GOAL_PROBE_COUNT] {
    [
        "What does done look like (observable outcome)?",
        "How will we verify (tests, commands, evidence)?",
        "What must not change (boundaries)?",
        "Who is the human owner, and when should we stop and ask?",
        "What Linear issue / GitHub PR / related work should we log against?",
    ]
}

/// One answered probe.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GoalProbe {
    /// Question shown to the human.
    pub question: String,
    /// Non-empty answer.
    pub answer: String,
}

/// Exactly five answered probes. Invalid states are unrepresentable.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GoalProbeSet {
    probes: [GoalProbe; GOAL_PROBE_COUNT],
}

/// Why a probe set could not be built.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ProbeError {
    /// Fewer or more than five answers.
    #[error("exactly {GOAL_PROBE_COUNT} probing answers are required (got {0})")]
    Count(usize),
    /// An answer was blank.
    #[error("probing answer {} is empty", .0 + 1)]
    Empty(usize),
}

impl GoalProbeSet {
    /// Builds a set from five (question, answer) pairs.
    pub fn try_new(pairs: Vec<(String, String)>) -> Result<Self, ProbeError> {
        if pairs.len() != GOAL_PROBE_COUNT {
            return Err(ProbeError::Count(pairs.len()));
        }
        let mut probes: Vec<GoalProbe> = Vec::with_capacity(GOAL_PROBE_COUNT);
        for (i, (question, answer)) in pairs.into_iter().enumerate() {
            if answer.trim().is_empty() {
                return Err(ProbeError::Empty(i));
            }
            probes.push(GoalProbe { question, answer: answer.trim().to_string() });
        }
        let probes: [GoalProbe; GOAL_PROBE_COUNT] = probes
            .try_into()
            .map_err(|v: Vec<_>| ProbeError::Count(v.len()))?;
        Ok(Self { probes })
    }

    /// Builds from five answers against the canonical questions.
    pub fn try_from_answers(answers: Vec<String>) -> Result<Self, ProbeError> {
        let questions = canonical_probe_questions();
        let pairs = answers
            .into_iter()
            .enumerate()
            .map(|(i, answer)| {
                let q = questions.get(i).copied().unwrap_or("").to_string();
                (q, answer)
            })
            .collect();
        Self::try_new(pairs)
    }

    /// Parses repeating `probe: answer` lines. Other lines stay in the headline.
    pub fn parse_from_text(text: &str) -> (String, Result<Self, ProbeError>) {
        let mut headline = Vec::new();
        let mut answers = Vec::new();
        for raw in text.lines() {
            let line = raw.trim();
            if line.is_empty() {
                continue;
            }
            if let Some((prefix, value)) = line.split_once(':')
                && prefix.trim().eq_ignore_ascii_case("probe")
            {
                answers.push(value.trim().to_string());
                continue;
            }
            headline.push(line.to_string());
        }
        (headline.join("\n"), Self::try_from_answers(answers))
    }

    /// Ordered probes.
    pub fn as_slice(&self) -> &[GoalProbe] {
        &self.probes
    }

    /// Renders a labelled block for the continuation prompt.
    pub fn render_block(&self) -> String {
        self.probes
            .iter()
            .enumerate()
            .map(|(i, p)| format!("{}. {}\n   {}", i + 1, p.question, p.answer))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

/// Best-in-class XML envelope around a user prompt or goal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PromptUpgrade {
    /// Wrapped XML body.
    pub xml: String,
}

impl PromptUpgrade {
    /// Wraps `text` (plus optional goal context) in `<aimee_prompt>` tags.
    /// Already-wrapped input is returned unchanged.
    pub fn wrap(text: &str, goal: Option<&GoalState>) -> Self {
        let trimmed = text.trim();
        if trimmed.contains("<aimee_prompt") {
            return Self { xml: trimmed.to_string() };
        }
        let mut xml = String::from("<aimee_prompt version=\"1\">\n");
        xml.push_str("  <intent>\n");
        xml.push_str(&indent(trimmed, 4));
        xml.push_str("\n  </intent>\n");
        if let Some(goal) = goal {
            xml.push_str("  <standing_goal>");
            xml.push_str(&escape_xml(&goal.goal));
            xml.push_str("</standing_goal>\n");
            if !goal.probes.is_empty() {
                xml.push_str("  <human_probes>\n");
                for probe in &goal.probes {
                    xml.push_str("    <probe question=\"");
                    xml.push_str(&escape_xml(&probe.question));
                    xml.push_str("\">");
                    xml.push_str(&escape_xml(&probe.answer));
                    xml.push_str("</probe>\n");
                }
                xml.push_str("  </human_probes>\n");
            }
        }
        xml.push_str("  <quality>\n");
        xml.push_str("    <tdd min_coverage=\"95\"/>\n");
        xml.push_str("    <review>greptile then coderabbit</review>\n");
        xml.push_str("  </quality>\n");
        xml.push_str("</aimee_prompt>");
        Self { xml }
    }
}

fn indent(text: &str, spaces: usize) -> String {
    let pad = " ".repeat(spaces);
    text.lines()
        .map(|line| format!("{pad}{}", escape_xml(line)))
        .collect::<Vec<_>>()
        .join("\n")
}

fn escape_xml(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

/// POST body for Linear connector `POST /v1/ensure-issue`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LinearEnsureRequest {
    /// Idempotency key (often `sess:<id>` or `aimee:<goal>`).
    pub kanban_task_id: String,
    /// Issue title.
    pub title: String,
    /// Markdown description.
    pub description: String,
    /// Hermes/Kanban status to project.
    pub hermes_status: String,
    /// Optional session id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
}

impl LinearEnsureRequest {
    /// Default connector base URL on this host.
    pub fn connector_url() -> &'static str {
        "http://127.0.0.1:8792/v1/ensure-issue"
    }

    /// Builds a request from a prompt headline.
    pub fn from_prompt(
        id: impl Into<String>,
        title: impl Into<String>,
        body: impl Into<String>,
    ) -> Self {
        Self {
            kanban_task_id: id.into(),
            title: title.into(),
            description: body.into(),
            hermes_status: "triage".into(),
            session_id: None,
        }
    }
}

/// Linear GraphQL `issueCreate` body (token stays in the `Authorization` header).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LinearGraphqlIssue {
    /// GraphQL document.
    pub query: String,
    /// Variables (`input.teamId`, `title`, `description`).
    pub variables: serde_json::Value,
}

impl LinearGraphqlIssue {
    /// Linear GraphQL endpoint.
    pub fn url() -> &'static str {
        "https://api.linear.app/graphql"
    }

    /// SWC team on this host (`~/.config/hermes-linear/config.yaml`).
    pub fn default_team_id() -> &'static str {
        "5371e279-c76c-4e84-b8df-254bf5fbfc27"
    }

    /// Builds an `issueCreate` mutation. Does not include the API key.
    pub fn issue_create(team_id: &str, title: &str, description: &str) -> Self {
        Self {
            query: "mutation($input: IssueCreateInput!) { issueCreate(input: $input) { success issue { identifier url } } }".into(),
            variables: serde_json::json!({
                "input": {
                    "teamId": team_id,
                    "title": title,
                    "description": description,
                }
            }),
        }
    }
}

/// CoT × GoT call against the self-hosted Drop MCP.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ThoughtGraphRequest {
    /// Goal or prompt to reason about.
    pub goal: String,
    /// `cot`, `got`, or `hybrid`.
    pub mode: String,
}

impl ThoughtGraphRequest {
    /// Drop MCP HTTP URL on this host.
    pub fn drop_mcp_url() -> &'static str {
        "http://127.0.0.1:7788/mcp"
    }

    /// Hybrid CoT×GoT request.
    pub fn hybrid(goal: impl Into<String>) -> Self {
        Self { goal: goal.into(), mode: "hybrid".into() }
    }
}

/// Quality gates expected after every loop turn.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QualityPolicy {
    /// Minimum line coverage percent (TDD).
    pub min_coverage_percent: u8,
    /// Greptile CLI must review before push.
    pub greptile_before_push: bool,
    /// CodeRabbit GitHub Action comments/fixes on the PR.
    pub coderabbit_on_pr: bool,
}

impl Default for QualityPolicy {
    fn default() -> Self {
        Self {
            min_coverage_percent: 95,
            greptile_before_push: true,
            coderabbit_on_pr: true,
        }
    }
}

/// Conventional commit + PR plan derived from a prompt (no secrets).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitHubPrPlan {
    /// `fix/` or `feat/` branch slug.
    pub branch: String,
    /// Conventional commit subject.
    pub commit_message: String,
    /// PR title.
    pub title: String,
}

impl GitHubPrPlan {
    /// Builds a plan. `fix` vs `feat` from the headline.
    pub fn from_prompt(prompt: &str) -> Self {
        let headline = prompt
            .lines()
            .find(|l| !l.trim().is_empty())
            .unwrap_or("loop update")
            .trim();
        let lower = headline.to_ascii_lowercase();
        let kind = if lower.contains("fix") || lower.contains("bug") {
            "fix"
        } else {
            "feat"
        };
        let slug: String = headline
            .chars()
            .map(|c| {
                if c.is_ascii_alphanumeric() {
                    c.to_ascii_lowercase()
                } else {
                    '-'
                }
            })
            .collect();
        let slug = slug.trim_matches('-');
        let slug = if slug.is_empty() { "loop" } else { slug };
        let slug: String = slug.chars().take(48).collect();
        Self {
            branch: format!("{kind}/{slug}"),
            commit_message: format!("{kind}: {headline}"),
            title: format!("{kind}: {headline}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_probe_set_requires_exactly_five_answers() {
        let actual = GoalProbeSet::try_from_answers(vec!["a".into(), "b".into()]);
        assert_eq!(actual, Err(ProbeError::Count(2)));
    }

    #[test]
    fn test_probe_set_rejects_empty_answer() {
        let fixture = vec!["a", "b", "c", "", "e"]
            .into_iter()
            .map(str::to_string)
            .collect();
        let actual = GoalProbeSet::try_from_answers(fixture);
        assert_eq!(actual, Err(ProbeError::Empty(3)));
    }

    #[test]
    fn test_probe_set_parses_probe_lines() {
        let fixture = "Ship the PWA\nprobe: installs\nprobe: cargo test\nprobe: no rebrand\nprobe: oveshen / stop on spend\nprobe: SPE-1";
        let (headline, set) = GoalProbeSet::parse_from_text(fixture);
        let actual = set.unwrap();
        let expected_headline = "Ship the PWA";
        assert_eq!(headline, expected_headline);
        assert_eq!(actual.as_slice().len(), GOAL_PROBE_COUNT);
        assert_eq!(actual.as_slice()[0].answer, "installs");
    }

    #[test]
    fn test_prompt_upgrade_wraps_xml_once() {
        let fixture = "Implement /goal loops";
        let actual = PromptUpgrade::wrap(fixture, None);
        assert!(actual.xml.starts_with("<aimee_prompt version=\"1\">"));
        assert!(actual.xml.contains("<intent>"));
        let again = PromptUpgrade::wrap(&actual.xml, None);
        assert_eq!(again.xml, actual.xml);
    }

    #[test]
    fn test_unlimited_tool_failures_is_not_three() {
        let actual = unlimited_tool_failures();
        assert_ne!(actual, 3);
        assert_eq!(actual, usize::MAX);
    }

    #[test]
    fn test_linear_ensure_request_points_at_connector() {
        let actual = LinearEnsureRequest::from_prompt("aimee:goal-1", "Ship PWA", "body");
        let expected = LinearEnsureRequest {
            kanban_task_id: "aimee:goal-1".into(),
            title: "Ship PWA".into(),
            description: "body".into(),
            hermes_status: "triage".into(),
            session_id: None,
        };
        assert_eq!(actual, expected);
        assert!(LinearEnsureRequest::connector_url().contains("8792"));
    }

    #[test]
    fn test_linear_graphql_issue_omits_token() {
        let actual = LinearGraphqlIssue::issue_create("team-1", "Title", "Body");
        assert_eq!(actual.variables["input"]["teamId"], "team-1");
        assert!(!serde_json::to_string(&actual).unwrap().contains("Bearer"));
        assert!(LinearGraphqlIssue::url().contains("api.linear.app"));
    }

    #[test]
    fn test_thought_graph_hybrid_uses_drop_mcp() {
        let actual = ThoughtGraphRequest::hybrid("Ship PWA");
        let expected = ThoughtGraphRequest { goal: "Ship PWA".into(), mode: "hybrid".into() };
        assert_eq!(actual, expected);
        assert!(ThoughtGraphRequest::drop_mcp_url().contains("7788"));
    }

    #[test]
    fn test_quality_policy_requires_95_and_review_clis() {
        let actual = QualityPolicy::default();
        let expected = QualityPolicy {
            min_coverage_percent: 95,
            greptile_before_push: true,
            coderabbit_on_pr: true,
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_github_pr_plan_classifies_fix() {
        let actual = GitHubPrPlan::from_prompt("Fix the parser crash");
        assert!(actual.branch.starts_with("fix/"));
        assert!(actual.commit_message.starts_with("fix:"));
    }
}
