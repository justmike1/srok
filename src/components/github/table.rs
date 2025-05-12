use crate::components::PagedTable;
use crate::services::github::models::CommitSearchResponse;
use leptos::prelude::*;
use std::sync::Arc;

#[derive(Clone, Debug, Default)]
struct GithubEntry {
    author: String,
    sha: String,
    html_url: String,
    api_key_hint: String,
}

#[component]
pub fn GithubTable(response: CommitSearchResponse) -> impl IntoView {
    let total = response.total_count as usize;

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
        <PagedTable
            entries=Arc::new(entries)
            total=total
            header=|| view! {
                <tr>
                    <th>"Author"</th>
                    <th>"Commit"</th>
                    <th>"Link"</th>
                    <th>"API Key Snippet"</th>
                </tr>
            }
            row=move |entry: &GithubEntry| view! {
                <tr>
                    <td>{entry.author.clone()}</td>
                    <td>{entry.sha.chars().take(7).collect::<String>()}</td>
                    <td>
                        <a
                            href=entry.html_url.clone()
                            target="_blank"
                            rel="noopener noreferrer"
                            style="color: #0ea5e9; text-decoration: underline;"
                        >
                            "View"
                        </a>
                    </td>
                    <td>
                        <code>{entry.api_key_hint.clone()}</code>
                    </td>
                </tr>
            }
        />
    }
}
