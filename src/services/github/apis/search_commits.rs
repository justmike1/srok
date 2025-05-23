/*
 * GitHub Commit Search
 *
 * Minimal spec exposing only the /search/commits endpoint from GitHub v3 REST API.
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

use super::{configuration, ContentType, Error};
use crate::{services::github::apis::ResponseContent, services::github::models};
use reqwest;
use serde::{de::Error as _, Deserialize, Serialize};

/// struct for passing parameters to the method [`search_commits`]
#[derive(Clone, Debug)]
pub struct SearchCommitsParams {
    /// Search keywords and qualifiers (e.g. `shodan_api_key+remove`).
    pub q: String,
    /// Must be `application/vnd.github.cloak-preview+json` for commit search.
    pub accept: String,
    /// Sort by `author-date` or `committer-date`.
    pub sort: Option<String>,
    /// `desc` (default) or `asc`.
    pub order: Option<String>,
    /// Results per page (max 100).
    pub per_page: Option<i32>,
    /// Page number of the results to fetch.
    pub page: Option<i32>,
}

/// struct for typed errors of method [`search_commits`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SearchCommitsError {
    UnknownValue(serde_json::Value),
}

pub async fn search_commits(
    configuration: &configuration::Configuration,
    params: SearchCommitsParams,
) -> Result<models::CommitSearchResponse, Error<SearchCommitsError>> {
    let uri_str = format!("{}/search/commits", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("q", &params.q.to_string())]);
    if let Some(ref param_value) = params.sort {
        req_builder = req_builder.query(&[("sort", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.order {
        req_builder = req_builder.query(&[("order", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.per_page {
        req_builder = req_builder.query(&[("per_page", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.page {
        req_builder = req_builder.query(&[("page", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.header("Accept", params.accept.to_string());

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::CommitSearchResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::CommitSearchResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<SearchCommitsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}
