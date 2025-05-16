#![cfg(feature = "ssr")]

use std::sync::OnceLock;

use axum::http::StatusCode;
use log::debug;
use quick_cache::sync::Cache;
use serde_json::Value;

use crate::constants::get_github_token;
use crate::integrations::Integration;
use crate::services::github::apis::configuration::Configuration;
use crate::services::github::apis::search_commits::{self, SearchCommitsParams};
use crate::services::github::models::{CommitSearchResponse, GithubError};
use crate::utils::cache::{get_or_init_cache, insert_into_cache, try_get_from_cache};

static GITHUB_CLIENT: OnceLock<Configuration> = OnceLock::new();
static GITHUB_CACHE: OnceLock<Cache<String, Value>> = OnceLock::new();

fn get_client() -> &'static Configuration {
    GITHUB_CLIENT.get_or_init(|| {
        let token = get_github_token().to_string();

        let mut config = Configuration::default();
        config.bearer_access_token = Some(token);
        config.user_agent = Some("github-client".to_string());

        config
    })
}

fn get_cache() -> &'static Cache<String, Value> {
    get_or_init_cache(&GITHUB_CACHE)
}

pub async fn search_commits_internal(
    query: &str,
    page: usize,
) -> Result<CommitSearchResponse, Box<dyn std::error::Error>> {
    let trimmed = query.trim();
    if trimmed.is_empty() {
        return Err("Search query cannot be empty".into());
    }

    let config = get_client();
    let params = SearchCommitsParams {
        q: trimmed.to_string(),
        accept: "application/vnd.github.cloak-preview+json".to_string(),
        sort: None,
        order: None,
        per_page: Some(100),
        page: page.try_into().ok(),
    };

    let raw = search_commits::search_commits(config, params)
        .await
        .map_err(|e| {
            debug!("GitHub API error: {e}");
            Box::new(e) as Box<dyn std::error::Error>
        })?;

    let json = serde_json::to_value(&raw).map_err(|e| {
        debug!("Serialization error: {e}");
        Box::new(e) as Box<dyn std::error::Error>
    })?;

    let deserialized: CommitSearchResponse = serde_json::from_value(json.clone()).map_err(|e| {
        debug!("Deserialization error: {e}\nRaw JSON: {json}");
        Box::new(e) as Box<dyn std::error::Error>
    })?;

    Ok(deserialized)
}

pub async fn search_integration(
    _client: &reqwest::Client,
    page: usize,
    integration: Integration,
) -> Result<CommitSearchResponse, GithubError> {
    let query = integration.to_github_query();
    debug!("Searching GitHub with query: {}", query);

    let query_key = format!("{}::page-{}", query, page);
    let cache = get_cache();

    if let Some(cached) = try_get_from_cache::<CommitSearchResponse>(cache, &query_key) {
        debug!("Found cached GitHub response");
        return Ok(cached);
    }

    match search_commits_internal(&query, page).await {
        Ok(response) => {
            insert_into_cache(cache, &query_key, &response);
            Ok(response)
        }
        Err(e) => Err(GithubError {
            message: e.to_string(),
            status: StatusCode::BAD_GATEWAY,
        }),
    }
}
