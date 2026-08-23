use std::sync::Arc;

use aimee_app::GrpcInfra;
use aimee_domain::{FuzzySearchRepository, SearchMatch};
use anyhow::{Context, Result};
use async_trait::async_trait;

use crate::proto_generated::FuzzySearchRequest;
use crate::proto_generated::aimee_service_client::AimeeServiceClient;

/// gRPC implementation of FuzzySearchRepository
pub struct AimeeFuzzySearchRepository<I> {
    infra: Arc<I>,
}

impl<I> AimeeFuzzySearchRepository<I> {
    /// Create a new repository with the given infrastructure
    ///
    /// # Arguments
    /// * `infra` - Infrastructure that provides gRPC connection
    pub fn new(infra: Arc<I>) -> Self {
        Self { infra }
    }
}

#[async_trait]
impl<I: GrpcInfra> FuzzySearchRepository for AimeeFuzzySearchRepository<I> {
    async fn fuzzy_search(
        &self,
        needle: &str,
        haystack: &str,
        search_all: bool,
    ) -> Result<Vec<SearchMatch>> {
        // Create gRPC request
        let request = tonic::Request::new(FuzzySearchRequest {
            needle: needle.to_string(),
            haystack: haystack.to_string(),
            search_all,
        });

        // Call gRPC API
        let channel = self.infra.channel()?;
        let mut client = AimeeServiceClient::new(channel);
        let response = client
            .fuzzy_search(request)
            .await
            .context("Failed to call FuzzySearch gRPC")?
            .into_inner();

        // Convert proto matches to domain matches
        let matches = response
            .matches
            .into_iter()
            .map(|m| SearchMatch { start_line: m.start_line, end_line: m.end_line })
            .collect();

        Ok(matches)
    }
}
