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
    /// Assistant text streamed during the current turn. The persisted
    /// conversation can lag the stream, so loop continuation reads this
    /// buffer first instead of re-fetching.
    pub turn_reply: String,
    /// Tool events observed during the current turn (calls, inputs,
    /// outputs). A turn with activity but no prose still made progress.
    pub turn_activity: u32,
}

impl UIState {
    pub fn new(env: Environment) -> Self {
        Self {
            cwd: env.cwd.clone(),
            conversation_id: Default::default(),
            goal: GoalStore::load(env.goal_path()),
            workflow: None,
            turn_reply: String::new(),
            turn_activity: 0,
        }
    }
}
