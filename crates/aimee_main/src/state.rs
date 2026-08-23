use std::path::PathBuf;

use aimee_api::{ConversationId, Environment};
use aimee_domain::{GoalStore, WorkflowRun};
use derive_setters::Setters;

//TODO: UIState and AimeePrompt seem like the same thing and can be merged
/// State information for the UI
#[derive(Debug, Clone, Setters)]
#[setters(strip_option)]
pub struct UIState {
    pub cwd: PathBuf,
    pub conversation_id: Option<ConversationId>,
    /// Standing `/goal` loop persisted under `~/.aimee/goal.json`.
    pub goal: GoalStore,
    /// In-flight multi-agent workflow (`/team run`).
    pub workflow: Option<WorkflowRun>,
}

impl UIState {
    pub fn new(env: Environment) -> Self {
        Self {
            cwd: env.cwd.clone(),
            conversation_id: Default::default(),
            goal: GoalStore::load(env.goal_path()),
            workflow: None,
        }
    }
}
