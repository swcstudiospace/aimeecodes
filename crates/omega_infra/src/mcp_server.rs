use std::collections::BTreeMap;

use omega_app::McpServerInfra;
use omega_domain::{Environment, McpServerConfig};

use crate::mcp_client::OmegaMcpClient;

#[derive(Clone)]
pub struct OmegaMcpServer;

#[async_trait::async_trait]
impl McpServerInfra for OmegaMcpServer {
    type Client = OmegaMcpClient;

    async fn connect(
        &self,
        config: McpServerConfig,
        env_vars: &BTreeMap<String, String>,
        environment: &Environment,
    ) -> anyhow::Result<Self::Client> {
        Ok(OmegaMcpClient::new(config, env_vars, environment.clone()))
    }
}
