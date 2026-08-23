use std::collections::BTreeMap;

use aimee_app::McpServerInfra;
use aimee_domain::{Environment, McpServerConfig};

use crate::mcp_client::AimeeMcpClient;

#[derive(Clone)]
pub struct AimeeMcpServer;

#[async_trait::async_trait]
impl McpServerInfra for AimeeMcpServer {
    type Client = AimeeMcpClient;

    async fn connect(
        &self,
        config: McpServerConfig,
        env_vars: &BTreeMap<String, String>,
        environment: &Environment,
    ) -> anyhow::Result<Self::Client> {
        Ok(AimeeMcpClient::new(config, env_vars, environment.clone()))
    }
}
