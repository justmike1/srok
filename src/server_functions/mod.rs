use leptos::server_fn::error::ServerFnError;
use leptos::*;

#[cfg(feature = "ssr")]
use crate::integrations::Integration;
#[cfg(feature = "ssr")]
use crate::services::github::search_integration as github_search;
#[cfg(feature = "ssr")]
use crate::services::ros::ResultErrorDTO;
use crate::services::ros::ResultRO;
#[cfg(feature = "ssr")]
use crate::services::shodan::search_integration as shodan_search;
#[cfg(feature = "ssr")]
use reqwest::Client;
use serde_json::Value;

#[server]
pub async fn search_integration(tool: String) -> Result<ResultRO<Value>, ServerFnError<String>> {
    #[cfg(feature = "ssr")]
    {
        let integration = Integration::from_name(&tool)
            .ok_or_else(|| ServerFnError::ServerError(format!("Unknown integration: {tool}")))?;

        let client = Client::new();

        let result: Result<serde_json::Value, ServerFnError<String>> = if integration.is_tool() {
            shodan_search(&client, integration)
                .await
                .map_err(|e| ServerFnError::ServerError(format!("Shodan search failed: {e}")))
                .and_then(|val| {
                    serde_json::to_value(val).map_err(|e| ServerFnError::ServerError(e.to_string()))
                })
        } else if integration.is_secret() {
            github_search(&client, integration)
                .await
                .map_err(|e| ServerFnError::ServerError(format!("GitHub search failed: {e}")))
                .and_then(|val| {
                    serde_json::to_value(val).map_err(|e| ServerFnError::ServerError(e.to_string()))
                })
        } else {
            return Ok(ResultRO {
                success: false,
                error: Some(format!("Integration `{tool}` not supported for search.")),
                error_object: Some(ResultErrorDTO {
                    code: Some("unsupported_tool".to_string()),
                    message: Some("Only Shodan or GitHub are supported".to_string()),
                }),
                ..Default::default()
            });
        };

        match result {
            Ok(json) => Ok(ResultRO {
                success: true,
                result: Some(json),
                ..Default::default()
            }),
            Err(e) => Ok(ResultRO {
                success: false,
                error: Some(e.to_string()),
                error_object: Some(ResultErrorDTO {
                    code: Some("search_failure".to_string()),
                    message: Some(e.to_string()),
                }),
                ..Default::default()
            }),
        }
    }

    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::ServerError(
            "Not available on client.".into(),
        ))
    }
}
