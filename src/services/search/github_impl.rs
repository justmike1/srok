use crate::integrations::Integration;
use crate::services::ros::ResultRO;
use crate::services::search::integration_service::IntegrationSearchService;
use async_trait::async_trait;
use reqwest::Client;
use serde_json::Value;

#[cfg(feature = "ssr")]
use crate::services::github::search_integration;
#[cfg(feature = "ssr")]
use crate::utils::to_result_ro;

pub struct GithubSearch(pub Integration);

#[async_trait]
impl IntegrationSearchService for GithubSearch {
    #[cfg(feature = "ssr")]
    async fn search(&self, client: &Client, page: usize) -> Result<ResultRO<Value>, String> {
        let response = search_integration(client, page, self.0.clone())
            .await
            .map_err(|e| e.to_string())?;

        to_result_ro(
            &response,
            page,
            response.items.len(),
            response.total_count as usize,
        )
    }

    #[cfg(not(feature = "ssr"))]
    async fn search(&self, _client: &Client, _page: usize) -> Result<ResultRO<Value>, String> {
        Err("GitHub search not available on client".into())
    }
}
