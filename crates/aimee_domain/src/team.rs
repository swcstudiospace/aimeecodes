use derive_setters::Setters;
use serde::{Deserialize, Serialize};

use crate::AgentId;

/// Role a member plays on a team.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum TeamRole {
    /// Coordinates the team and assigns work.
    Lead,
    /// Executes assigned steps.
    #[default]
    Member,
    /// Reviews outputs before they land.
    Reviewer,
}

/// One agent participating in a team.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Setters)]
#[setters(strip_option, into)]
pub struct TeamMember {
    /// Display name.
    pub name: String,
    /// Agent id this member runs as (`aimee`, `muse`, `sage`, or custom).
    pub agent_id: AgentId,
    /// Role on the team.
    pub role: TeamRole,
}

impl TeamMember {
    /// Creates a member with the given name and agent.
    pub fn new(name: impl Into<String>, agent_id: impl Into<AgentId>) -> Self {
        Self {
            name: name.into(),
            agent_id: agent_id.into(),
            role: TeamRole::Member,
        }
    }
}

/// A named roster of agents that can run workflows together.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Setters, Default)]
#[setters(strip_option, into)]
pub struct Team {
    /// Team identifier (`engineering`, `review`).
    pub name: String,
    /// Members in roster order.
    pub members: Vec<TeamMember>,
}

impl Team {
    /// Creates an empty team.
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), members: Vec::new() }
    }

    /// Adds a member and returns the team (builder style).
    pub fn with_member(mut self, member: TeamMember) -> Self {
        self.members.push(member);
        self
    }

    /// Returns the lead, if any.
    pub fn lead(&self) -> Option<&TeamMember> {
        self.members
            .iter()
            .find(|member| member.role == TeamRole::Lead)
    }

    /// Loop team: Muse plans, Aimee orchestrates, Sage reviews.
    pub fn engineering() -> Self {
        Self::new("engineering")
            .with_member(TeamMember::new("lead", AgentId::MUSE).role(TeamRole::Lead))
            .with_member(TeamMember::new("orchestrator", AgentId::AIMEE))
            .with_member(TeamMember::new("review", AgentId::SAGE).role(TeamRole::Reviewer))
    }

    /// Frontend specialists. Aimee remains orchestrator/lead.
    pub fn frontend() -> Self {
        Self::new("frontend")
            .with_member(TeamMember::new("orchestrator", AgentId::AIMEE).role(TeamRole::Lead))
            .with_member(TeamMember::new("interface", AgentId::new("fe-ui")))
            .with_member(TeamMember::new("web3-ux", AgentId::new("fe-web3")))
            .with_member(TeamMember::new("realtime", AgentId::new("fe-realtime")))
            .with_member(TeamMember::new("edge", AgentId::new("fe-edge")))
            .with_member(TeamMember::new("quality", AgentId::new("fe-qa")).role(TeamRole::Reviewer))
    }

    /// Backend specialists. Aimee remains orchestrator/lead.
    pub fn backend() -> Self {
        Self::new("backend")
            .with_member(TeamMember::new("orchestrator", AgentId::AIMEE).role(TeamRole::Lead))
            .with_member(TeamMember::new("services", AgentId::new("be-api")))
            .with_member(TeamMember::new("web3", AgentId::new("be-web3")))
            .with_member(TeamMember::new("data", AgentId::new("be-data")))
            .with_member(TeamMember::new("security", AgentId::new("be-security")))
            .with_member(
                TeamMember::new("reliability", AgentId::new("be-reliability"))
                    .role(TeamRole::Reviewer),
            )
    }

    /// Platform specialists. Aimee remains orchestrator/lead.
    pub fn platform() -> Self {
        Self::new("platform")
            .with_member(TeamMember::new("orchestrator", AgentId::AIMEE).role(TeamRole::Lead))
            .with_member(TeamMember::new("kubernetes", AgentId::new("plat-k8s")))
            .with_member(TeamMember::new("cloud", AgentId::new("plat-cloud")))
            .with_member(TeamMember::new("sre", AgentId::new("plat-sre")))
            .with_member(
                TeamMember::new("compliance", AgentId::new("plat-compliance"))
                    .role(TeamRole::Reviewer),
            )
    }

    /// All built-in engineering org teams.
    pub fn engineering_org() -> Vec<Self> {
        vec![
            Self::engineering(),
            Self::frontend(),
            Self::backend(),
            Self::platform(),
        ]
    }
}

/// One step in a multi-agent workflow.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Setters)]
#[setters(strip_option, into)]
pub struct WorkflowStep {
    /// Step name.
    pub name: String,
    /// Agent that executes the step.
    pub agent_id: AgentId,
    /// Prompt template handed to that agent.
    pub prompt: String,
}

impl WorkflowStep {
    /// Creates a workflow step.
    pub fn new(
        name: impl Into<String>,
        agent_id: impl Into<AgentId>,
        prompt: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            agent_id: agent_id.into(),
            prompt: prompt.into(),
        }
    }
}

/// Ordered multi-agent workflow (Hermes/Agno-style).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Setters, Default)]
#[setters(strip_option, into)]
pub struct AgentWorkflow {
    /// Workflow identifier.
    pub name: String,
    /// Optional team this workflow binds to.
    #[serde(default)]
    pub team: Option<String>,
    /// Ordered steps.
    pub steps: Vec<WorkflowStep>,
}

impl AgentWorkflow {
    /// Creates an empty workflow.
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), team: None, steps: Vec::new() }
    }

    /// Adds a step.
    pub fn with_step(mut self, step: WorkflowStep) -> Self {
        self.steps.push(step);
        self
    }

    /// Renders a human-readable plan.
    pub fn render(&self) -> String {
        let mut out = format!("Workflow: {}\n", self.name);
        if let Some(team) = &self.team {
            out.push_str("Team: ");
            out.push_str(team);
            out.push('\n');
        }
        for (index, step) in self.steps.iter().enumerate() {
            out.push_str(&format!(
                "{}. [{}] {} — {}\n",
                index + 1,
                step.agent_id.as_str(),
                step.name,
                step.prompt
            ));
        }
        out
    }

    /// Built-in muse → aimee → sage shipping workflow.
    pub fn engineering_ship() -> Self {
        Self::new("ship")
            .team("engineering".to_string())
            .with_step(WorkflowStep::new(
                "plan",
                AgentId::MUSE,
                "Outline the work. Be concrete about files and acceptance checks.",
            ))
            .with_step(WorkflowStep::new(
                "build",
                AgentId::AIMEE,
                "Implement the plan. Verify with the stack's checks. Dispatch specialists when the work is in one lane.",
            ))
            .with_step(WorkflowStep::new(
                "review",
                AgentId::SAGE,
                "Critique the result. Name remaining risks.",
            ))
    }

    /// Frontend team ship: Aimee orchestrates UI specialists then Sage reviews.
    pub fn frontend_ship() -> Self {
        Self::new("frontend-ship")
            .team("frontend".to_string())
            .with_step(WorkflowStep::new(
                "build-ui",
                AgentId::new("fe-ui"),
                "Implement the interface. Match tokens and a11y.",
            ))
            .with_step(WorkflowStep::new(
                "verify-ui",
                AgentId::new("fe-qa"),
                "Prove the UI with tests, keyboard paths, and residual risk.",
            ))
            .with_step(WorkflowStep::new(
                "review",
                AgentId::SAGE,
                "Review frontend change. Call out a11y and WEB3 UX risks.",
            ))
    }

    /// Backend team ship: API then security review.
    pub fn backend_ship() -> Self {
        Self::new("backend-ship")
            .team("backend".to_string())
            .with_step(WorkflowStep::new(
                "build-api",
                AgentId::new("be-api"),
                "Implement the service. Clean architecture. Verify.",
            ))
            .with_step(WorkflowStep::new(
                "secure",
                AgentId::new("be-security"),
                "Authorize mutations. No secrets. FedRAMP/SOC2-minded controls.",
            ))
            .with_step(WorkflowStep::new(
                "review",
                AgentId::SAGE,
                "Review backend change. Name remaining risks.",
            ))
    }

    /// Platform team ship: cluster then compliance evidence.
    pub fn platform_ship() -> Self {
        Self::new("platform-ship")
            .team("platform".to_string())
            .with_step(WorkflowStep::new(
                "cluster",
                AgentId::new("plat-k8s"),
                "Apply cluster/GitOps change with least privilege.",
            ))
            .with_step(WorkflowStep::new(
                "compliance",
                AgentId::new("plat-compliance"),
                "Map controls. Report SOC2/FedRAMP gaps with evidence.",
            ))
            .with_step(WorkflowStep::new(
                "review",
                AgentId::SAGE,
                "Review platform change. Name remaining risks.",
            ))
    }
}

/// In-flight execution of an [`AgentWorkflow`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkflowRun {
    /// Workflow being executed.
    pub workflow: AgentWorkflow,
    /// Index of the step currently running.
    pub index: usize,
}

impl WorkflowRun {
    /// Starts a run at step 0. Returns `None` when the workflow has no steps.
    pub fn start(workflow: AgentWorkflow) -> Option<Self> {
        if workflow.steps.is_empty() {
            None
        } else {
            Some(Self { workflow, index: 0 })
        }
    }

    /// Step currently executing.
    pub fn current(&self) -> Option<&WorkflowStep> {
        self.workflow.steps.get(self.index)
    }

    /// Human-readable progress label (`1/3 plan`).
    pub fn progress_label(&self) -> String {
        let name = self
            .current()
            .map(|step| step.name.as_str())
            .unwrap_or("done");
        format!(
            "{}/{} {name}",
            self.index.saturating_add(1),
            self.workflow.steps.len()
        )
    }

    /// Prompt injected for the current step.
    pub fn step_prompt(&self) -> Option<String> {
        let step = self.current()?;
        Some(format!(
            "[Workflow {} — step {}]\nAgent: {}\nTask: {}\n\n{}",
            self.workflow.name,
            self.progress_label(),
            step.agent_id.as_str(),
            step.name,
            step.prompt
        ))
    }

    /// Advances to the next step. Returns `false` when the run is finished.
    pub fn advance(&mut self) -> bool {
        self.index = self.index.saturating_add(1);
        self.index < self.workflow.steps.len()
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_team_lead_and_workflow_render() {
        let fixture = Team::new("engineering")
            .with_member(TeamMember::new("lead", AgentId::MUSE).role(TeamRole::Lead))
            .with_member(TeamMember::new("impl", AgentId::AIMEE));
        let actual_lead = fixture.lead().map(|m| m.agent_id.as_str());
        assert_eq!(actual_lead, Some("muse"));

        let workflow = AgentWorkflow::new("ship-pwa")
            .team("engineering".to_string())
            .with_step(WorkflowStep::new("plan", AgentId::MUSE, "outline the PWA"))
            .with_step(WorkflowStep::new("build", AgentId::AIMEE, "implement"));
        let actual = workflow.render();
        assert!(actual.contains("Workflow: ship-pwa"));
        assert!(actual.contains("[muse] plan"));
        assert!(actual.contains("[aimee] build"));
    }

    #[test]
    fn test_workflow_run_advances_then_finishes() {
        let mut run = WorkflowRun::start(AgentWorkflow::engineering_ship()).unwrap();
        assert_eq!(run.current().unwrap().name, "plan");
        assert!(run.advance());
        assert_eq!(run.current().unwrap().name, "build");
        assert!(run.advance());
        assert_eq!(run.current().unwrap().name, "review");
        let actual = run.advance();
        let expected = false;
        assert_eq!(actual, expected);
        assert!(run.current().is_none());
    }

    #[test]
    fn test_engineering_org_has_three_delivery_teams() {
        let actual = Team::engineering_org();
        let names: Vec<&str> = actual.iter().map(|t| t.name.as_str()).collect();
        assert_eq!(
            names,
            vec!["engineering", "frontend", "backend", "platform"]
        );
        assert_eq!(Team::frontend().members.len(), 6);
        assert_eq!(Team::backend().members.len(), 6);
        assert_eq!(Team::platform().members.len(), 5);
        assert_eq!(
            Team::frontend().lead().map(|m| m.agent_id.as_str()),
            Some("aimee")
        );
        assert!(AgentWorkflow::frontend_ship().render().contains("[fe-ui]"));
        assert!(
            AgentWorkflow::backend_ship()
                .render()
                .contains("[be-security]")
        );
        assert!(
            AgentWorkflow::platform_ship()
                .render()
                .contains("[plat-compliance]")
        );
    }
}
