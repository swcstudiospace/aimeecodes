use std::sync::Arc;

use omega_domain::{
    Conversation, EndPayload, EventData, EventHandle, ResponsePayload,
};

use crate::{CheckpointKind, SessionPathwayService};

/// Orchestrator hook that logs pathway checkpoints on agent response and turn end.
///
/// Logging is best-effort unless the underlying service is configured with
/// `hard_fail`. Failures are returned so the orchestrator can decide policy.
pub struct PathwayLogHook<S, K, E> {
    service: Arc<SessionPathwayService<S, K, E>>,
    agent_id: Option<String>,
    log_responses: bool,
    log_turn_end: bool,
}

impl<S, K, E> PathwayLogHook<S, K, E> {
    /// Creates a hook that logs both responses and turn ends.
    pub fn new(service: Arc<SessionPathwayService<S, K, E>>) -> Self {
        Self {
            service,
            agent_id: None,
            log_responses: true,
            log_turn_end: true,
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
}

#[async_trait::async_trait]
impl<S, K, E> EventHandle<EventData<ResponsePayload>> for PathwayLogHook<S, K, E>
where
    S: crate::PathwayStore + 'static,
    K: crate::KipBackend + 'static,
    E: crate::EternalStore + 'static,
{
    async fn handle(
        &self,
        _event: &EventData<ResponsePayload>,
        conversation: &mut Conversation,
    ) -> anyhow::Result<()> {
        if !self.log_responses {
            return Ok(());
        }
        self.service
            .log_checkpoint(
                conversation.clone(),
                CheckpointKind::AgentResponse,
                self.agent_id.clone(),
            )
            .await
            .map_err(|e| anyhow::anyhow!(e))?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl<S, K, E> EventHandle<EventData<EndPayload>> for PathwayLogHook<S, K, E>
where
    S: crate::PathwayStore + 'static,
    K: crate::KipBackend + 'static,
    E: crate::EternalStore + 'static,
{
    async fn handle(
        &self,
        _event: &EventData<EndPayload>,
        conversation: &mut Conversation,
    ) -> anyhow::Result<()> {
        if !self.log_turn_end {
            return Ok(());
        }
        self.service
            .log_checkpoint(
                conversation.clone(),
                CheckpointKind::TurnEnd,
                self.agent_id.clone(),
            )
            .await
            .map_err(|e| anyhow::anyhow!(e))?;
        Ok(())
    }
}
