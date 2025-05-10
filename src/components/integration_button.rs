use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

#[component]
pub fn IntegrationButton(name: &'static str, logo: &'static str) -> impl IntoView {
    let navigate = use_navigate();

    view! {
        <button
            class="integration-button"
            on:click=move |_| {
                navigate(&format!("/integration/{}", name), Default::default());
            }
        >
            <img src=logo alt={format!("{name} logo")} class="integration-logo" />
            <span class="integration-name">{name}</span>
        </button>
    }
}
