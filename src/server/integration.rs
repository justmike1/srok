use leptos::server_fn::error::ServerFnError;
use leptos::*;

use crate::services::ros::ResultRO;
use serde_json::Value;

#[cfg(feature = "ssr")]
use reqwest::Client;

#[cfg(feature = "ssr")]
use crate::integrations::Integration;
#[cfg(feature = "ssr")]
use crate::services::ros::ResultErrorDTO;
#[cfg(feature = "ssr")]
use crate::services::search::{GithubSearch, IntegrationSearchService, ShodanSearch};

#[server]
pub async fn search_integration(tool: String) -> Result<ResultRO<Value>, ServerFnError<String>> {
    #[cfg(feature = "ssr")]
    {
        let integration = Integration::from_name(&tool)
            .ok_or_else(|| ServerFnError::ServerError(format!("Unknown integration: {tool}")))?;

        let client = Client::new();

        let service: Box<dyn IntegrationSearchService + Send + Sync> = if integration.is_tool() {
            Box::new(ShodanSearch(integration))
        } else if integration.is_secret() {
            Box::new(GithubSearch(integration))
        } else {
            return Ok(ResultRO {
                success: false,
                error: Some(format!("Integration `{tool}` not supported")),
                error_object: Some(ResultErrorDTO {
                    code: Some("unsupported_tool".into()),
                    message: Some("Only Shodan or GitHub supported".into()),
                }),
                ..Default::default()
            });
        };

        service
            .search(&client)
            .await
            .map_err(|e| ServerFnError::ServerError(e.to_string()))
    }

    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::ServerError("Client only".into()))
    }
}
