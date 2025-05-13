use crate::components::github::table::GithubTable;
use crate::server::integration::search_integration;
use crate::services::github::models::CommitSearchResponse;
use crate::services::ros::PagingRO;
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

#[component]
pub fn GithubIntegrationPage() -> impl IntoView {
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
                                        Some(inner) => serde_json::from_value::<CommitSearchResponse>(inner)
                                            .map(|pd| (pd, String::new(), paging))
                                            .unwrap_or_else(|e| (
                                                CommitSearchResponse {
                                                    total_count: 0,
                                                    incomplete_results: false,
                                                    items: vec![],
                                                },
                                                e.to_string(),
                                                paging_clone,
                                            )),
                                        None => (
                                            CommitSearchResponse {
                                                total_count: 0,
                                                incomplete_results: false,
                                                items: vec![],
                                            },
                                            ro.error.unwrap_or("Missing result".to_string()),
                                            paging,
                                        )
                                    }
                                }
                                Err(err) => (
                                    CommitSearchResponse {
                                        total_count: 0,
                                        incomplete_results: false,
                                        items: vec![],
                                    },
                                    err.to_string(),
                                    PagingRO::default(),
                                ),
                            };

                            let hidden = err_msg.is_empty();
                            view! {
                                <div>
                                    <GithubTable
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
