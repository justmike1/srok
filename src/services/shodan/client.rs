#![cfg(feature = "ssr")]

use std::sync::OnceLock;

use axum::http::StatusCode;
use log::debug;
use quick_cache::sync::Cache;
use serde_json::Value;

use crate::integrations::Integration;
use crate::services::shodan::apis::configuration::{ApiKey, Configuration};
use crate::services::shodan::apis::search_host::{search_host, SearchHostParams};
use crate::services::shodan::models::{ShodanError, ShodanSearchResponse};

static CLIENT: OnceLock<Configuration> = OnceLock::new();
static SHODAN_CACHE: OnceLock<Cache<String, Value>> = OnceLock::new();

fn get_client() -> &'static Configuration {
    CLIENT.get_or_init(|| {
        let api_key =
            std::env::var("SHODAN_API_KEY").expect("SHODAN_API_KEY environment variable not set");

        let mut config = Configuration::default();
        config.api_key = Some(ApiKey {
            prefix: None,
            key: api_key,
        });
        config
    })
}

fn get_cache() -> &'static Cache<String, Value> {
    SHODAN_CACHE.get_or_init(|| Cache::new(3600))
}

pub async fn search_hosts(query: &str) -> Result<ShodanSearchResponse, Box<dyn std::error::Error>> {
    let trimmed = query.trim();
    if trimmed.is_empty() {
        return Err("Search query cannot be empty".into());
    }

    let config = get_client();
    let key = config
        .api_key
        .as_ref()
        .expect("Shodan API key must be set")
        .key
        .clone();

    let params = SearchHostParams {
        key,
        query: trimmed.to_string(),
        facets: None,
        page: None,
    };

    let raw = search_host(config, params).await.map_err(|e| {
        debug!("Shodan API error: {}", e);
        Box::new(e) as Box<dyn std::error::Error>
    })?;

    serde_json::from_value(raw.clone()).map_err(|e| {
        debug!("Deserialization error: {e}\nRaw JSON: {raw}");
        Box::new(e) as Box<dyn std::error::Error>
    })
}

pub async fn search_integration(
    _client: &reqwest::Client,
    integration: Integration,
) -> Result<ShodanSearchResponse, ShodanError> {
    let query = integration.to_shodan_query();
    let query_key = query.to_string();

    debug!("Searching Shodan with query: {}", query_key);

    if let Some(cached) = get_cache().get(&query_key) {
        debug!("Cache hit for query: {}", query_key);
        return Ok(serde_json::from_value(cached.clone()).unwrap());
    }

    match search_hosts(&query_key).await {
        Ok(response) => {
            let json = serde_json::to_value(&response).map_err(|e| ShodanError {
                message: e.to_string(),
                status: StatusCode::INTERNAL_SERVER_ERROR,
            })?;
            get_cache().insert(query_key.clone(), json.clone());
            Ok(serde_json::from_value(json).unwrap())
        }
        Err(e) => Err(ShodanError {
            message: e.to_string(),
            status: StatusCode::BAD_GATEWAY,
        }),
    }
}
