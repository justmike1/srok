use crate::components::github::integration_layout::GithubIntegrationPage;
use crate::components::shodan::integration_layout::ShodanIntegrationPage;
use crate::integrations::Integration;
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

#[component]
pub fn IntegrationPage() -> impl IntoView {
    let params = use_params_map();
    let tool = move || params.with(|p| p.get("tool").into_iter().next().unwrap_or_default());

    let integration =
        Memo::new(move |_| Integration::from_name(&tool()).unwrap_or(Integration::Shodan));

    view! {
        <Show
            when=move || integration.get().is_secret()
            fallback=|| view! { <ShodanIntegrationPage/> }
        >
            <GithubIntegrationPage/>
        </Show>
    }
}
