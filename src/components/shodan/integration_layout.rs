use crate::components::shodan::table::ShodanTable;
use crate::server::integration::search_integration;
use crate::services::ros::PagingRO;
use crate::services::shodan::models::search_response::ShodanSearchResponse;
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

#[component]
pub fn ShodanIntegrationPage() -> impl IntoView {
    let (page, set_page) = signal(0usize);
    let params = use_params_map();

    let tool = move || params.with(|p| p.get("tool").into_iter().next().unwrap_or_default());
    let tool_signal = Memo::new(move |_| tool());

    let fetch_action = Action::new(move |(tool, page): &(String, usize)| {
        let tool = tool.clone();
        let page = *page;
        async move { search_integration(tool, page).await }
    });

    Effect::new(move |_| {
        let current_tool = tool_signal.get();
        let current_page = page.get();
        if !current_tool.is_empty() {
            fetch_action.dispatch((current_tool.clone(), current_page));
        }
    });

    let on_page_change = Callback::new(move |new_page: usize| {
        set_page.set(new_page);
        let current_tool = tool_signal.get();
        if !current_tool.is_empty() {
            fetch_action.dispatch((current_tool.clone(), new_page));
        }
    });

    view! {
        <section class="integration-page">
            <h1 class="integration-page-title">
                "Integration: " {move || tool_signal.get()}
            </h1>

            <Transition fallback=move || view! { <p>"Loading..."</p> }>
                { move || {
                    fetch_action.value().with(|maybe_result| {
                        maybe_result.as_ref().map(|result| {
                            let (table_data, err_msg, paging) = match result {
                                Ok(json) => {
                                    let ro = json.clone();
                                    let paging = ro.paging.clone().unwrap_or_default();
                                    let paging_clone = paging.clone();
                                    match ro.result {
                                        Some(inner) => serde_json::from_value::<ShodanSearchResponse>(inner)
                                            .map(|pd| (pd, String::new(), paging))
                                            .unwrap_or_else(|e| (
                                                ShodanSearchResponse {
                                                    matches: vec![],
                                                    total: 0,
                                                    facets: None,
                                                },
                                                e.to_string(),
                                                paging_clone,
                                            )),
                                        None => (
                                            ShodanSearchResponse {
                                                matches: vec![],
                                                total: 0,
                                                facets: None,
                                            },
                                            ro.error.unwrap_or("Missing result".to_string()),
                                            paging,
                                        )
                                    }
                                }
                                Err(err) => (
                                    ShodanSearchResponse {
                                        matches: vec![],
                                        total: 0,
                                        facets: None,
                                    },
                                    err.to_string(),
                                    PagingRO::default(),
                                ),
                            };

                            let hidden = err_msg.is_empty();

                            view! {
                                <div>
                                    <ShodanTable
                                        response={table_data}
                                        paging={paging}
                                        page={page}
                                        set_page={set_page}
                                        on_page_change={on_page_change}
                                    />
                                    <p class="error" hidden=move || hidden>
                                        {move || err_msg.clone()}
                                    </p>
                                </div>
                            }
                        })
                    })
                }}
            </Transition>
        </section>
    }
}
