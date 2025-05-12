use crate::components::github::table::GithubTable;
use crate::server_functions::search_integration;
use crate::services::github::models::CommitSearchResponse;
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

#[component]
pub fn GithubIntegrationPage() -> impl IntoView {
    let params = use_params_map();
    let tool = move || params.with(|p| p.get("tool").into_iter().next().unwrap_or_default());

    let fetch_action = Action::new(move |tool: &String| {
        let tool = tool.clone();
        async move { search_integration(tool).await }
    });

    let tool_signal = Memo::new(move |_| tool());
    Effect::new(move |_| {
        let tool = tool_signal.get();
        if !tool.is_empty() {
            fetch_action.dispatch(tool);
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
                      let (table_data, err_msg) = match result {
                          Ok(json) => {
                            let ro = json.clone();
                              match ro.result {
                                  Some(inner) => serde_json::from_value::<CommitSearchResponse>(inner)
                                      .map(|pd| (pd, String::new()))
                                      .unwrap_or_else(|e| (
                                          CommitSearchResponse {
                                              total_count: 0,
                                              incomplete_results: false,
                                              items: vec![],
                                          },
                                          e.to_string(),
                                      )),
                                  None => (
                                      CommitSearchResponse {
                                          total_count: 0,
                                          incomplete_results: false,
                                          items: vec![],
                                      },
                                      ro.error.unwrap_or("Missing result".to_string()),
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
                          ),
                      };

                      let hidden = err_msg.is_empty();
                      view! {
                          <div>
                              <GithubTable response={table_data} />
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
