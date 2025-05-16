#![cfg(feature = "ssr")]

use std::sync::OnceLock;

use axum::http::StatusCode;
use log::debug;
use quick_cache::sync::Cache;
use serde_json::Value;

use crate::constants::get_shodan_token;
use crate::integrations::Integration;
use crate::services::shodan::apis::configuration::{ApiKey, Configuration};
use crate::services::shodan::apis::search_host::{search_host, SearchHostParams};
use crate::services::shodan::models::{ShodanError, ShodanSearchResponse};
use crate::utils::cache::{get_or_init_cache, insert_into_cache, try_get_from_cache};

static SHODAN_CLIENT: OnceLock<Configuration> = OnceLock::new();
static SHODAN_CACHE: OnceLock<Cache<String, Value>> = OnceLock::new();

fn get_client() -> &'static Configuration {
    SHODAN_CLIENT.get_or_init(|| {
        let api_key = get_shodan_token().to_string();

        let mut config = Configuration::default();
        config.api_key = Some(ApiKey {
            prefix: None,
            key: api_key,
        });
        config
    })
}

fn get_cache() -> &'static Cache<String, Value> {
    get_or_init_cache(&SHODAN_CACHE)
}

pub async fn search_hosts(
    query: &str,
    page: usize,
) -> Result<ShodanSearchResponse, Box<dyn std::error::Error>> {
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
        page: page.try_into().ok(),
    };

    let raw = search_host(config, params).await.map_err(|e| {
        debug!("Shodan API error: {}", e);
        Box::new(e) as Box<dyn std::error::Error>
    })?;

    Ok(raw)
}

pub async fn search_integration(
    _client: &reqwest::Client,
    page: usize,
    integration: Integration,
) -> Result<ShodanSearchResponse, ShodanError> {
    let query = integration.to_shodan_query();
    debug!("Searching Shodan with query: {}", query);

    let query_key = format!("{}::page-{}", query, page);
    let cache = get_cache();

    if let Some(cached) = try_get_from_cache::<ShodanSearchResponse>(cache, &query_key) {
        debug!("Cache hit for query: {}", query_key);
        return Ok(cached);
    }

    match search_hosts(&query, page).await {
        Ok(response) => {
            insert_into_cache(cache, &query_key, &response);
            Ok(response)
        }
        Err(e) => Err(ShodanError {
            message: e.to_string(),
            status: StatusCode::BAD_GATEWAY,
        }),
    }
}
