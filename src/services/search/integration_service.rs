use crate::services::ros::ResultRO;
use async_trait::async_trait;
use reqwest::Client;
use serde_json::Value;

#[async_trait]
pub trait IntegrationSearchService {
    async fn search(&self, client: &Client) -> Result<ResultRO<Value>, String>;
}
