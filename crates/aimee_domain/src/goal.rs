use std::path::{Path, PathBuf};

use derive_setters::Setters;
use serde::{Deserialize, Serialize};

/// No auto-pause. `/goal` loops until the judge or the user stops them.
pub const UNLIMITED_TURNS: u32 = u32::MAX;

/// Lifecycle of a standing `/goal` loop.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum GoalStatus {
    /// The loop should auto-continue after each turn.
    #[default]
    Active,
    /// Parked until `/goal resume`.
    Paused,
    /// Judge or user marked the goal complete.
    Done,
    /// Explicitly cleared; no continuation prompt.
    Cleared,
}

/// Optional structured completion contract parsed from `field: value` lines.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, Setters)]
#[setters(strip_option, into)]
pub struct GoalContract {
    /// Desired outcome.
    #[serde(default)]
    pub outcome: String,
    /// How to verify done.
    #[serde(default)]
    pub verification: String,
    /// Hard constraints.
    #[serde(default)]
    pub constraints: String,
    /// Files / areas that must not change.
    #[serde(default)]
    pub boundaries: String,
    /// When the loop must stop and ask the user.
    #[serde(default)]
    pub stop_when: String,
}

impl GoalContract {
    /// Returns true when every field is empty.
    pub fn is_empty(&self) -> bool {
        self.outcome.trim().is_empty()
            && self.verification.trim().is_empty()
            && self.constraints.trim().is_empty()
            && self.boundaries.trim().is_empty()
            && self.stop_when.trim().is_empty()
    }

    /// Renders non-empty fields as a labelled block.
    pub fn render_block(&self) -> String {
        let mut lines = Vec::new();
        for (label, value) in [
            ("Outcome", self.outcome.as_str()),
            ("Verification", self.verification.as_str()),
            ("Constraints", self.constraints.as_str()),
            ("Boundaries", self.boundaries.as_str()),
            ("Stop when", self.stop_when.as_str()),
        ] {
            let value = value.trim();
            if !value.is_empty() {
                lines.push(format!("- {label}: {value}"));
            }
        }
        lines.join("\n")
    }
}

/// Persistent standing-goal state (Hermes `/goal` contract, minus the judge).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Setters)]
#[setters(strip_option, into)]
pub struct GoalState {
    /// Headline objective.
    pub goal: String,
    /// Loop status.
    pub status: GoalStatus,
    /// Turns consumed since the goal was set.
    pub turns_used: u32,
    /// Soft budget before auto-pause. `UNLIMITED_TURNS` never auto-pauses.
    pub max_turns: u32,
    /// Extra criteria appended via `/subgoal`.
    #[serde(default)]
    pub subgoals: Vec<String>,
    /// Optional structured contract.
    #[serde(default)]
    pub contract: GoalContract,
    /// Isolated workspace id for untrusted agent work (`aimee pod up --id`).
    #[serde(default)]
    pub pod_id: Option<String>,
    /// Pull request URL once `/goal pr` succeeds.
    #[serde(default)]
    pub pr_url: Option<String>,
    /// Mandatory HITL probes (five answers) gathered after `/goal` is written.
    #[serde(default)]
    pub probes: Vec<crate::GoalProbe>,
}

impl GoalState {
    /// Creates an active goal from free-form text (and optional contract lines).
    pub fn new(text: impl Into<String>) -> Self {
        let raw = text.into();
        let (headline, contract) = parse_contract(&raw);
        Self {
            goal: if headline.is_empty() {
                raw.trim().to_string()
            } else {
                headline
            },
            status: GoalStatus::Active,
            turns_used: 0,
            max_turns: UNLIMITED_TURNS,
            subgoals: Vec::new(),
            contract,
            pod_id: None,
            pr_url: None,
            probes: Vec::new(),
        }
    }

    /// Returns true when `text` is already a standing-goal continuation.
    pub fn is_continuation(text: &str) -> bool {
        text.contains("[Continuing toward your standing goal]")
    }

    /// Returns true when the loop should inject a continuation prompt.
    pub fn should_continue(&self) -> bool {
        self.status == GoalStatus::Active && !self.goal.trim().is_empty()
    }

    /// Increments the turn counter and auto-pauses when the budget is spent.
    pub fn tick(&mut self) {
        if self.status != GoalStatus::Active {
            return;
        }
        self.turns_used = self.turns_used.saturating_add(1);
        if self.max_turns != UNLIMITED_TURNS && self.turns_used >= self.max_turns {
            self.status = GoalStatus::Paused;
        }
    }

    /// Hermes-style continuation preamble injected before the next user turn.
    pub fn continuation_prompt(&self) -> String {
        let mut out = format!(
            "[Continuing toward your standing goal]\nGoal: {}\n",
            self.goal
        );
        if !self.subgoals.is_empty() {
            out.push_str("Subgoals:\n");
            for item in &self.subgoals {
                out.push_str("- ");
                out.push_str(item);
                out.push('\n');
            }
        }
        let contract = self.contract.render_block();
        if !contract.is_empty() {
            out.push_str("Contract:\n");
            out.push_str(&contract);
            out.push('\n');
        }
        if !self.probes.is_empty() {
            out.push_str("Human probes:\n");
            for (i, probe) in self.probes.iter().enumerate() {
                out.push_str(&format!(
                    "{}. {}\n   {}\n",
                    i + 1,
                    probe.question,
                    probe.answer
                ));
            }
        }
        out.push_str(
            "Upgrade this turn as best-in-class XML (<aimee_prompt>). Use Drop MCP CoT×GoT (reason_cot_got, hybrid) before high-stakes steps. TDD 95% coverage; Greptile before push; CodeRabbit on the PR.\n",
        );
        if let Some(pod_id) = self.pod_id.as_deref() {
            out.push_str("Sandbox pod: ");
            out.push_str(pod_id);
            out.push_str(" (aimee pod ssh ");
            out.push_str(pod_id);
            out.push_str(")\n");
        }
        if let Some(pr_url) = self.pr_url.as_deref() {
            out.push_str("Pull request: ");
            out.push_str(pr_url);
            out.push('\n');
        }
        out.push_str(
            "Continue working until: (1) the task is complete, AND (2) you have verified the result.\nWhen every criterion is verified, end with a line: GOAL_COMPLETE: <reason>",
        );
        out
    }
}

/// Verdict from the deterministic `/goal` judge.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GoalVerdict {
    /// True when the loop should stop.
    pub complete: bool,
    /// Short reason shown to the user.
    pub reason: String,
}

/// Inspects the last assistant reply. Fail-open: no marker means continue.
pub fn judge_goal(state: &GoalState, last_reply: &str) -> GoalVerdict {
    let reply = last_reply.trim();
    if reply.is_empty() {
        return GoalVerdict { complete: false, reason: "empty reply".into() };
    }
    if let Some(reason) = extract_goal_complete(reply) {
        return GoalVerdict { complete: true, reason };
    }
    let stop = state.contract.stop_when.trim();
    if !stop.is_empty()
        && reply
            .to_ascii_lowercase()
            .contains(&stop.to_ascii_lowercase())
    {
        return GoalVerdict { complete: true, reason: format!("stop_when matched: {stop}") };
    }
    GoalVerdict { complete: false, reason: "no completion marker".into() }
}

fn extract_goal_complete(reply: &str) -> Option<String> {
    for line in reply.lines() {
        let line = line.trim();
        if let Some((prefix, rest)) = line.split_once(':')
            && prefix.trim().eq_ignore_ascii_case("goal_complete")
        {
            let reason = rest.trim();
            return Some(if reason.is_empty() {
                "GOAL_COMPLETE".into()
            } else {
                reason.to_string()
            });
        }
        if line.eq_ignore_ascii_case("goal_complete") {
            return Some("GOAL_COMPLETE".into());
        }
    }
    None
}

/// Splits user-typed goal text into a headline plus `field: value` contract.
///
/// Recognized prefixes: `outcome`, `verify`/`verification`, `constraints`,
/// `boundaries`, `stop when`/`stop_when`. Unrecognized colon lines stay in the
/// headline so `Fix bug: the parser` is not mangled.
pub fn parse_contract(text: &str) -> (String, GoalContract) {
    let mut headline = Vec::new();
    let mut contract = GoalContract::default();
    for raw in text.lines() {
        let line = raw.trim();
        if line.is_empty() {
            continue;
        }
        if let Some((prefix, value)) = line.split_once(':') {
            let key = prefix.trim().to_ascii_lowercase();
            let value = value.trim();
            if !value.is_empty() {
                match key.as_str() {
                    "outcome" => contract.outcome = join_field(&contract.outcome, value),
                    "verify" | "verification" => {
                        contract.verification = join_field(&contract.verification, value)
                    }
                    "constraints" => {
                        contract.constraints = join_field(&contract.constraints, value)
                    }
                    "boundaries" => contract.boundaries = join_field(&contract.boundaries, value),
                    "stop when" | "stop_when" => {
                        contract.stop_when = join_field(&contract.stop_when, value)
                    }
                    _ => headline.push(line.to_string()),
                }
                continue;
            }
        }
        headline.push(line.to_string());
    }
    (headline.join(" "), contract)
}

fn join_field(existing: &str, value: &str) -> String {
    if existing.is_empty() {
        value.to_string()
    } else {
        format!("{existing} {value}")
    }
}

/// File-backed store for the active standing goal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GoalStore {
    /// Persistence path (`~/.aimee/goal.json`).
    pub path: PathBuf,
    state: Option<GoalState>,
}

impl GoalStore {
    /// Loads a store from `path`, ignoring a missing or corrupt file.
    pub fn load(path: impl Into<PathBuf>) -> Self {
        let path = path.into();
        let state = std::fs::read_to_string(&path)
            .ok()
            .and_then(|raw| serde_json::from_str(&raw).ok());
        Self { path, state }
    }

    /// Empty store pointing at `path` (does not touch the filesystem).
    pub fn empty(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into(), state: None }
    }

    /// Returns the current goal, if any.
    pub fn current(&self) -> Option<&GoalState> {
        self.state.as_ref()
    }

    /// Sets a new active goal and persists it.
    pub fn set(&mut self, text: impl Into<String>) -> anyhow::Result<&GoalState> {
        self.state = Some(GoalState::new(text));
        self.persist()?;
        Ok(self.state.as_ref().expect("just set"))
    }

    /// Sets a goal only after five HITL probes are answered.
    pub fn set_loop(
        &mut self,
        text: impl Into<String>,
        probes: crate::GoalProbeSet,
    ) -> anyhow::Result<&GoalState> {
        let mut state = GoalState::new(text);
        state.probes = probes.as_slice().to_vec();
        self.state = Some(state);
        self.persist()?;
        Ok(self.state.as_ref().expect("just set"))
    }

    /// Adds a subgoal to the active goal.
    ///
    /// # Errors
    ///
    /// Returns an error when no goal is active.
    pub fn add_subgoal(&mut self, text: impl Into<String>) -> anyhow::Result<&GoalState> {
        let state = self
            .state
            .as_mut()
            .ok_or_else(|| anyhow::anyhow!("No active goal. Set one with /goal <text>."))?;
        state.subgoals.push(text.into());
        self.persist()?;
        Ok(self.state.as_ref().expect("present"))
    }

    /// Records the isolated workspace id for this goal.
    ///
    /// # Errors
    ///
    /// Returns when no goal is active.
    pub fn attach_pod(&mut self, pod_id: impl Into<String>) -> anyhow::Result<&GoalState> {
        let state = self
            .state
            .as_mut()
            .ok_or_else(|| anyhow::anyhow!("No active goal. Set one with /goal <text>."))?;
        state.pod_id = Some(pod_id.into());
        self.persist()?;
        Ok(self.state.as_ref().expect("present"))
    }

    /// Records a pull-request URL opened for this goal.
    ///
    /// # Errors
    ///
    /// Returns when no goal is active.
    pub fn attach_pr(&mut self, url: impl Into<String>) -> anyhow::Result<&GoalState> {
        let state = self
            .state
            .as_mut()
            .ok_or_else(|| anyhow::anyhow!("No active goal. Set one with /goal <text>."))?;
        state.pr_url = Some(url.into());
        self.persist()?;
        Ok(self.state.as_ref().expect("present"))
    }

    /// Pauses the active goal.
    pub fn pause(&mut self) -> anyhow::Result<&GoalState> {
        self.map_status(GoalStatus::Paused)
    }

    /// Resumes a paused or done goal.
    pub fn resume(&mut self) -> anyhow::Result<&GoalState> {
        self.map_status(GoalStatus::Active)
    }

    /// Marks the goal done.
    pub fn done(&mut self) -> anyhow::Result<&GoalState> {
        self.map_status(GoalStatus::Done)
    }

    /// Clears the goal from memory and disk.
    pub fn clear(&mut self) -> anyhow::Result<()> {
        self.state = None;
        if self.path.exists() {
            std::fs::remove_file(&self.path)?;
        }
        Ok(())
    }

    /// Ticks the active goal after a completed agent turn.
    pub fn tick(&mut self) -> anyhow::Result<Option<&GoalState>> {
        if let Some(state) = self.state.as_mut() {
            state.tick();
            self.persist()?;
        }
        Ok(self.state.as_ref())
    }

    /// Continuation prompt when the goal is active.
    pub fn continuation_prompt(&self) -> Option<String> {
        self.state
            .as_ref()
            .filter(|goal| goal.should_continue())
            .map(GoalState::continuation_prompt)
    }

    /// Judges the last assistant reply and marks the goal done when complete.
    pub fn judge(&mut self, last_reply: &str) -> anyhow::Result<Option<GoalVerdict>> {
        let Some(state) = self.state.as_ref() else {
            return Ok(None);
        };
        if !state.should_continue() {
            return Ok(None);
        }
        let verdict = judge_goal(state, last_reply);
        if verdict.complete {
            self.map_status(GoalStatus::Done)?;
        }
        Ok(Some(verdict))
    }

    fn map_status(&mut self, status: GoalStatus) -> anyhow::Result<&GoalState> {
        let state = self
            .state
            .as_mut()
            .ok_or_else(|| anyhow::anyhow!("No active goal. Set one with /goal <text>."))?;
        state.status = status;
        self.persist()?;
        Ok(self.state.as_ref().expect("present"))
    }

    fn persist(&self) -> anyhow::Result<()> {
        if let Some(parent) = self.path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        if let Some(state) = &self.state {
            std::fs::write(&self.path, serde_json::to_string_pretty(state)?)?;
        }
        Ok(())
    }

    /// Returns the default on-disk path for a goal store.
    pub fn default_path(base: &Path) -> PathBuf {
        base.join("goal.json")
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_parse_contract_keeps_incidental_colons() {
        let (actual, contract) = parse_contract("Fix bug: the parser");
        let expected = "Fix bug: the parser".to_string();
        assert_eq!(actual, expected);
        assert!(contract.is_empty());
    }

    #[test]
    fn test_parse_contract_extracts_known_fields() {
        let fixture = "Migrate auth to JWT\nverify: tests pass\nconstraints: keep /login shape";
        let (actual_headline, actual) = parse_contract(fixture);
        let expected = GoalContract::default()
            .verification("tests pass")
            .constraints("keep /login shape");
        assert_eq!(actual_headline, "Migrate auth to JWT");
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_goal_continuation_includes_subgoals() {
        let mut fixture = GoalState::new("Ship PWA wallet login");
        fixture.subgoals.push("HITL spend stays off".into());
        let actual = fixture.continuation_prompt();
        assert!(actual.contains("[Continuing toward your standing goal]"));
        assert!(actual.contains("Ship PWA wallet login"));
        assert!(actual.contains("HITL spend stays off"));
    }

    #[test]
    fn test_goal_tick_auto_pauses_at_budget() {
        let mut fixture = GoalState::new("x");
        fixture.max_turns = 2;
        fixture.tick();
        fixture.tick();
        let actual = fixture.status;
        let expected = GoalStatus::Paused;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_goal_default_budget_is_unlimited() {
        let fixture = GoalState::new("Ship");
        let actual = (fixture.max_turns, fixture.status);
        let expected = (UNLIMITED_TURNS, GoalStatus::Active);
        assert_eq!(actual, expected);
        let mut ticking = fixture;
        ticking.tick();
        ticking.tick();
        assert_eq!(ticking.status, GoalStatus::Active);
        assert_eq!(ticking.turns_used, 2);
    }

    #[test]
    fn test_goal_store_round_trip() {
        let path = std::env::temp_dir().join(format!(
            "aimee-goal-{}-{}.json",
            std::process::id(),
            uuid::Uuid::new_v4()
        ));
        let mut store = GoalStore::empty(&path);
        store.set("Build teams").unwrap();
        store.add_subgoal("write tests").unwrap();
        let reloaded = GoalStore::load(&path);
        let actual = reloaded.current().cloned();
        let expected = store.current().cloned();
        let _ = std::fs::remove_file(&path);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_is_continuation_detects_marker() {
        let fixture = GoalState::new("x");
        let actual = GoalState::is_continuation(&fixture.continuation_prompt());
        assert!(actual);
        assert!(!GoalState::is_continuation("ordinary user text"));
    }

    #[test]
    fn test_judge_goal_fail_open_without_marker() {
        let fixture = GoalState::new("Ship the PWA");
        let actual = judge_goal(&fixture, "Still wiring the splash.");
        let expected = GoalVerdict { complete: false, reason: "no completion marker".into() };
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_judge_goal_completes_on_marker() {
        let fixture = GoalState::new("Ship the PWA");
        let actual = judge_goal(&fixture, "Verified splash.\nGOAL_COMPLETE: PWA installs");
        let expected = GoalVerdict { complete: true, reason: "PWA installs".into() };
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_judge_goal_honors_stop_when() {
        let fixture = GoalState::new("Ship\nstop when: tests pass");
        let actual = judge_goal(&fixture, "All tests pass on aimee_domain.");
        assert!(actual.complete);
    }

    #[test]
    fn test_goal_legacy_json_defaults_pod_and_pr() {
        let fixture = r#"{"goal":"Ship","status":"active","turns_used":0,"max_turns":30}"#;
        let actual: GoalState = serde_json::from_str(fixture).unwrap();
        let mut expected = GoalState::new("Ship");
        expected.max_turns = 30;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_continuation_includes_pod_id() {
        let mut fixture = GoalState::new("Ship PWA");
        fixture.pod_id = Some("aimee-ship-pwa".into());
        let actual = fixture.continuation_prompt();
        assert!(actual.contains("Sandbox pod: aimee-ship-pwa"));
    }
}
