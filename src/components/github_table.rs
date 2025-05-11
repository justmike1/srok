use crate::services::github::models::CommitSearchResponse;
use leptos::prelude::*;

#[derive(Clone)]
struct GithubEntry {
    author: String,
    sha: String,
    html_url: String,
    api_key_hint: String,
}

#[component]
pub fn GithubTable(response: CommitSearchResponse) -> impl IntoView {
    let entries: Vec<GithubEntry> = response
        .items
        .into_iter()
        .map(|item| {
            let author = item
                .commit
                .author
                .name
                .clone()
                .unwrap_or_else(|| "unknown".to_string());

            let sha = item.sha;
            let html_url = item.html_url;
            let message = item.commit.message;

            let api_key_hint = message
                .lines()
                .find(|line| {
                    let lower = line.to_lowercase();
                    lower.contains("key") || lower.contains("secret")
                })
                .unwrap_or("no api key reference")
                .to_string();

            GithubEntry {
                author,
                sha,
                html_url,
                api_key_hint,
            }
        })
        .collect();

    view! {
      <div class="table-container">
        <table class="leptos-datatable">
            <thead>
                <tr>
                    <th>"Author"</th>
                    <th>"Commit"</th>
                    <th>"Link"</th>
                    <th>"API Key Snippet"</th>
                </tr>
            </thead>
            <tbody>
                {entries.into_iter().map(|entry| {
                    let sha_short = entry.sha.chars().take(7).collect::<String>();
                    let url = entry.html_url.clone();

                    view! {
                        <tr>
                            <td>{entry.author}</td>
                            <td>{sha_short}</td>
                            <td>
                                <a
                                    href=url
                                    target="_blank"
                                    rel="noopener noreferrer"
                                    style="color: #0ea5e9; text-decoration: underline;"
                                >
                                    "View"
                                </a>
                            </td>
                            <td>
                                <code>{entry.api_key_hint}</code>
                            </td>
                        </tr>
                    }
                }).collect_view()}
            </tbody>
        </table>
      </div>
    }
}
