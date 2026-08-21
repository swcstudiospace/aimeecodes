use std::sync::Arc;

use aimee_domain::{Conversation, EndPayload, EventData, EventHandle, ResponsePayload};
use tracing::warn;

use crate::{CheckpointKind, EternalStore, KipBackend, PathwayStore, SessionPathwayService};

/// Orchestrator hook that logs pathway checkpoints on agent response and turn end.
///
/// By default failures are logged and swallowed (`hard_fail = false`) so pathway
/// durability never blocks the agent turn. Set `hard_fail` to surface errors.
#[derive(Clone)]
pub struct PathwayLogHook<S, K, E> {
    service: Arc<SessionPathwayService<S, K, E>>,
    agent_id: Option<String>,
    log_responses: bool,
    log_turn_end: bool,
    hard_fail: bool,
}

impl<S, K, E> PathwayLogHook<S, K, E> {
    /// Creates a hook that logs both responses and turn ends (best-effort).
    pub fn new(service: Arc<SessionPathwayService<S, K, E>>) -> Self {
        Self {
            service,
            agent_id: None,
            log_responses: true,
            log_turn_end: true,
            hard_fail: false,
        }
    }

    /// Sets the agent id recorded on checkpoints.
    pub fn agent_id(mut self, agent_id: impl Into<String>) -> Self {
        self.agent_id = Some(agent_id.into());
        self
    }

    /// Controls whether `on_response` events produce checkpoints.
    pub fn log_responses(mut self, enabled: bool) -> Self {
        self.log_responses = enabled;
        self
    }

    /// Controls whether `on_end` events produce checkpoints.
    pub fn log_turn_end(mut self, enabled: bool) -> Self {
        self.log_turn_end = enabled;
        self
    }

    /// When true, pathway errors fail the orchestrator event handler.
    pub fn hard_fail(mut self, enabled: bool) -> Self {
        self.hard_fail = enabled;
        self
    }
}

impl<S, K, E> PathwayLogHook<S, K, E>
where
    S: PathwayStore,
    K: KipBackend,
    E: EternalStore,
{
    async fn log(&self, conversation: &Conversation, kind: CheckpointKind) -> anyhow::Result<()> {
        match self
            .service
            .log_checkpoint(conversation.clone(), kind, self.agent_id.clone())
            .await
        {
            Ok(_) => Ok(()),
            Err(err) if self.hard_fail => Err(anyhow::anyhow!(err)),
            Err(err) => {
                warn!(error = %err, kind = %kind, "session pathway checkpoint failed");
                Ok(())
            }
        }
    }
}

#[async_trait::async_trait]
impl<S, K, E> EventHandle<EventData<ResponsePayload>> for PathwayLogHook<S, K, E>
where
    S: PathwayStore + 'static,
    K: KipBackend + 'static,
    E: EternalStore + 'static,
{
    async fn handle(
        &self,
        _event: &EventData<ResponsePayload>,
        conversation: &mut Conversation,
    ) -> anyhow::Result<()> {
        if !self.log_responses {
            return Ok(());
        }
        self.log(conversation, CheckpointKind::AgentResponse).await
    }
}

#[async_trait::async_trait]
impl<S, K, E> EventHandle<EventData<EndPayload>> for PathwayLogHook<S, K, E>
where
    S: PathwayStore + 'static,
    K: KipBackend + 'static,
    E: EternalStore + 'static,
{
    async fn handle(
        &self,
        _event: &EventData<EndPayload>,
        conversation: &mut Conversation,
    ) -> anyhow::Result<()> {
        if !self.log_turn_end {
            return Ok(());
        }
        self.log(conversation, CheckpointKind::TurnEnd).await
    }
}
