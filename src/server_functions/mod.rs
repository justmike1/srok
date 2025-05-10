use leptos::server_fn::error::ServerFnError;
use leptos::*;

#[cfg(feature = "ssr")]
use crate::integrations::Integration;
#[cfg(feature = "ssr")]
use crate::services::github::search_integration as github_search;
#[cfg(feature = "ssr")]
use crate::services::shodan::search_integration as shodan_search;
#[cfg(feature = "ssr")]
use reqwest::Client;

#[server]
pub async fn search_integration(tool: String) -> Result<String, ServerFnError<String>> {
    #[cfg(feature = "ssr")]
    {
        let integration = Integration::from_name(&tool)
            .ok_or_else(|| ServerFnError::ServerError(format!("Unknown integration: {tool}")))?;

        let client = Client::new();

        let json_value: serde_json::Value = if integration.is_tool() {
            let result = shodan_search(&client, integration)
                .await
                .map_err(|e| ServerFnError::ServerError(format!("Shodan search failed: {e}")))?;
            serde_json::to_value(result).map_err(|e| ServerFnError::ServerError(e.to_string()))?
        } else if integration.is_secret() {
            let result = github_search(&client, integration)
                .await
                .map_err(|e| ServerFnError::ServerError(format!("GitHub search failed: {e}")))?;
            serde_json::to_value(result).map_err(|e| ServerFnError::ServerError(e.to_string()))?
        } else {
            return Err(ServerFnError::ServerError(format!(
                "Integration `{tool}` not supported for search."
            )));
        };

        serde_json::to_string(&json_value).map_err(|e| ServerFnError::ServerError(e.to_string()))
    }

    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::ServerError(
            "Not available on client.".into(),
        ))
    }
}
