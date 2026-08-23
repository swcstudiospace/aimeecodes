use reqwest::Client;
use serde_json::{Value, json};

use crate::{AndaError, AndaResult, KipBackend, KipReceipt};

/// HTTP client for a Cognitive Nexus JSON-RPC endpoint.
///
/// Speaks the LDC Labs nexus shape:
/// `POST {base_url}/kip` with
/// `{"jsonrpc":"2.0","id":1,"method":"execute_kip","params":{"command":"..."}}`.
#[derive(Debug, Clone)]
pub struct NexusHttpBackend {
    client: Client,
    kip_url: String,
}

impl NexusHttpBackend {
    /// Creates a backend targeting `base_url` (e.g. `http://127.0.0.1:8091`).
    pub fn new(base_url: impl AsRef<str>) -> Self {
        let base = base_url.as_ref().trim_end_matches('/');
        Self { client: Client::new(), kip_url: format!("{base}/kip") }
    }

    /// Creates a backend with a custom reqwest client (timeouts, proxies, etc.).
    pub fn with_client(base_url: impl AsRef<str>, client: Client) -> Self {
        let base = base_url.as_ref().trim_end_matches('/');
        Self { client, kip_url: format!("{base}/kip") }
    }
}

#[async_trait::async_trait]
impl KipBackend for NexusHttpBackend {
    async fn execute_kip(&self, command: &str) -> AndaResult<KipReceipt> {
        let body = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "execute_kip",
            "params": { "command": command }
        });

        let response = self
            .client
            .post(&self.kip_url)
            .json(&body)
            .send()
            .await
            .map_err(|e| AndaError::Kip(e.to_string()))?;

        let status = response.status();
        let value: Value = response
            .json()
            .await
            .map_err(|e| AndaError::Kip(e.to_string()))?;

        if !status.is_success() {
            return Ok(KipReceipt::new(
                command,
                false,
                Some(format!("http {status}: {value}")),
            ));
        }

        if let Some(err) = value.get("error") {
            return Ok(KipReceipt::new(command, false, Some(err.to_string())));
        }

        let summary = value
            .get("result")
            .map(|r| r.to_string())
            .or_else(|| Some(value.to_string()));

        Ok(KipReceipt::new(command, true, summary))
    }
}

/// Builds a KML UPSERT that records a pathway checkpoint as an Event concept.
pub fn pathway_event_upsert(
    conversation_id: &str,
    seq: u64,
    content_hash: &str,
    kind: &str,
) -> String {
    let name = format!("aimee-pathway-{conversation_id}-seq-{seq}");
    format!(
        r#"UPSERT {{
  CONCEPT ?event {{
    {{ type: "Event", name: "{name}" }}
    SET ATTRIBUTES {{
      description: "Aimee session pathway checkpoint",
      conversation_id: "{conversation_id}",
      seq: {seq},
      content_hash: "{content_hash}",
      kind: "{kind}",
      source: "aimee_anda"
    }}
  }}
}}
WITH METADATA {{ source: "aimee:pathway:{conversation_id}:{seq}", author: "$self", confidence: 1.0 }}"#
    )
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_pathway_event_upsert_contains_hash() {
        let actual = pathway_event_upsert("cid", 3, "deadbeef", "agent_response");
        assert!(actual.contains("deadbeef"));
        assert!(actual.contains("seq: 3"));
        assert!(actual.contains("Event"));
        assert_eq!(actual.contains("UPSERT"), true);
    }
}
