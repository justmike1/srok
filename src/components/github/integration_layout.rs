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
        let api_page = page + 1;
        async move { search_integration(tool, api_page).await }
    });

    Effect::new(move |_| {
        let tool = tool_signal.get();
        let current_page = page.get();
        if !tool.is_empty() {
            fetch_action.dispatch((tool, current_page));
        }
    });

    let on_page_change = Callback::new({
        let set_page = set_page.clone();
        move |new_page: usize| {
            set_page.set(new_page);
        }
    });

    view! {
        <section class="integration-page">
            <h1 class="integration-page-title">
                "Integration: " {move || tool_signal.get()}
            </h1>

            <Transition fallback=move || view! { <p>"Loading..."</p> }>
                {move || {
                    fetch_action.value().get().map(|result| {
                        let (table_data, err_msg, paging) = match result {
                            Ok(json) => {
                                let paging = json.paging.clone().unwrap_or_default();
                                let paging_clone = paging.clone();

                                match json.result {
                                    Some(inner) => serde_json::from_value::<CommitSearchResponse>(inner)
                                        .map(|pd| (pd, String::new(), paging))
                                        .unwrap_or_else(|e| (
                                            CommitSearchResponse::default(),
                                            e.to_string(),
                                            paging_clone,
                                        )),
                                    None => (
                                        CommitSearchResponse::default(),
                                        json.error.unwrap_or("Missing result".to_string()),
                                        paging,
                                    )
                                }
                            }
                            Err(err) => (
                                CommitSearchResponse::default(),
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
                                    is_loading={fetch_action.pending()}
                                />
                                <p class="error" hidden=move || hidden>
                                    {move || err_msg.clone()}
                                </p>
                            </div>
                        }
                    })
                }}
            </Transition>
        </section>
    }
}
