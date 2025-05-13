use crate::integrations::Integration;
use crate::services::ros::ResultRO;
use crate::services::search::integration_service::IntegrationSearchService;
use async_trait::async_trait;
use reqwest::Client;
use serde_json::Value;

#[cfg(feature = "ssr")]
use crate::services::github::search_integration;
#[cfg(feature = "ssr")]
use crate::services::ros::PagingRO;

pub struct GithubSearch(pub Integration);

#[async_trait]
impl IntegrationSearchService for GithubSearch {
    #[cfg(feature = "ssr")]
    async fn search(&self, client: &Client, page: usize) -> Result<ResultRO<Value>, String> {
        let response = search_integration(client, page, self.0.clone())
            .await
            .map_err(|e| e.to_string())?;

        let per_page = response.items.len();
        let start = (page - 1) * per_page;
        let total = response.total_count as usize;
        let has_more = start + per_page < total;

        let paging = PagingRO {
            start: Some(start),
            limit: Some(per_page),
            total: Some(total as u64),
            has_more: Some(has_more),
        };

        let result_json = serde_json::to_value(response).map_err(|e| e.to_string())?;

        Ok(ResultRO {
            result: Some(result_json),
            paging: Some(paging),
            ..Default::default()
        })
    }

    #[cfg(not(feature = "ssr"))]
    async fn search(&self, _client: &Client, _page: usize) -> Result<ResultRO<Value>, String> {
        Err("GitHub search not available on client".into())
    }
}
