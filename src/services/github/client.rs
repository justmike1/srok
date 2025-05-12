#![cfg(feature = "ssr")]

use std::sync::OnceLock;

use axum::http::StatusCode;
use log::debug;
use quick_cache::sync::Cache;
use serde_json::Value;

use crate::integrations::Integration;
use crate::services::github::apis::configuration::Configuration;
use crate::services::github::apis::search_commits::{self, SearchCommitsParams};
use crate::services::github::models::{CommitSearchResponse, GithubError};

static CLIENT: OnceLock<Configuration> = OnceLock::new();
static GITHUB_CACHE: OnceLock<Cache<String, Value>> = OnceLock::new();

fn get_client() -> &'static Configuration {
    CLIENT.get_or_init(|| {
        let token =
            std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN environment variable not set");

        let mut config = Configuration::default();
        config.bearer_access_token = Some(token);
        config.user_agent = Some("velotix-github-client".to_string());

        config
    })
}

fn get_cache() -> &'static Cache<String, Value> {
    GITHUB_CACHE.get_or_init(|| Cache::new(3600))
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

    let query_key = query.to_string();

    if let Some(cached) = get_cache().get(&query_key) {
        debug!("Found cached GitHub response");
        return Ok(serde_json::from_value(cached.clone()).unwrap());
    }

    match search_commits_internal(&query, page).await {
        Ok(response) => {
            let json = serde_json::to_value(&response).map_err(|e| GithubError {
                message: e.to_string(),
                status: StatusCode::INTERNAL_SERVER_ERROR,
            })?;
            get_cache().insert(query_key, json.clone());
            Ok(serde_json::from_value(json).unwrap())
        }
        Err(e) => Err(GithubError {
            message: e.to_string(),
            status: StatusCode::BAD_GATEWAY,
        }),
    }
}
