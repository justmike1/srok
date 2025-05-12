use crate::integrations::Integration;
use crate::services::ros::ResultRO;
use crate::services::search::integration_service::IntegrationSearchService;
use async_trait::async_trait;
use reqwest::Client;
use serde_json::Value;

#[cfg(feature = "ssr")]
use crate::services::ros::PagingRO;
#[cfg(feature = "ssr")]
use crate::services::shodan::search_integration;

pub struct ShodanSearch(pub Integration);

#[async_trait]
impl IntegrationSearchService for ShodanSearch {
    #[cfg(feature = "ssr")]
    async fn search(&self, client: &Client, page: usize) -> Result<ResultRO<Value>, String> {
        let response = search_integration(client, page, self.0.clone())
            .await
            .map_err(|e| e.to_string())?;

        let paging = PagingRO {
            start: Some(0),
            limit: Some(response.matches.len()),
            total: Some(response.total),
            has_more: Some(response.total > response.matches.len() as u64),
        };

        let result_json = serde_json::to_value(response).map_err(|e| e.to_string())?;

        Ok(ResultRO {
            result: Some(result_json),
            paging: Some(paging),
            ..Default::default()
        })
    }

    #[cfg(not(feature = "ssr"))]
    async fn search(&self, _client: &Client, page: usize) -> Result<ResultRO<Value>, String> {
        Err("Shodan search not available on client".into())
    }
}
