use crate::components::PagedTable;
use crate::services::shodan::models::search_response::ShodanSearchResponse;
use leptos::prelude::*;
use std::sync::Arc;

#[derive(Clone)]
struct ShodanEntry {
    host_url: Option<String>,
    hostname: Option<String>,
    title: Option<String>,
    ip: String,
    port: Option<u16>,
    org: String,
    city: String,
    country: String,
}

#[component]
pub fn ShodanTable(response: ShodanSearchResponse) -> impl IntoView {
    let entries: Vec<ShodanEntry> = response
        .matches
        .into_iter()
        .map(|m| {
            let (city, country) = match m.location {
                Some(loc) => (
                    loc.city.unwrap_or_default(),
                    loc.country_name.unwrap_or_default(),
                ),
                None => ("".to_string(), "".to_string()),
            };

            let (host_url, title) = match m.http {
                Some(http) => (http.host, http.title),
                None => (None, None),
            };

            let hostname = m.hostnames.as_ref().and_then(|v| v.first().cloned());

            ShodanEntry {
                host_url,
                hostname,
                title,
                ip: m.ip_str,
                port: m.port,
                org: m.org.unwrap_or_default(),
                city,
                country,
            }
        })
        .collect();

    view! {
        <PagedTable
            entries=Arc::new(entries)
            header=|| view! {
                <tr>
                    <th>"IP"</th>
                    <th>"Port"</th>
                    <th>"Org"</th>
                    <th>"City"</th>
                    <th>"Country"</th>
                    <th colspan="2">"Actions"</th>
                </tr>
            }
            row=move |entry: &ShodanEntry| {
                let ip_ping = entry.ip.clone();
                let ip_creds = entry.ip.clone();
                let tooltip = entry
                    .hostname
                    .clone()
                    .or(entry.title.clone())
                    .or(entry.host_url.clone())
                    .unwrap_or_else(|| "no host".to_string());
                let redirect_host = entry.hostname.clone().map(|h| format!("http://{}", h));
                let style_host = redirect_host.clone();
                view! {
                    <tr
                        class="clickable-row"
                        title=tooltip.clone()
                        on:click=move |_| {
                            if let Some(url) = redirect_host.as_ref() {
                                window()
                                    .open_with_url_and_target(url, "_blank")
                                    .unwrap_or(None);
                            }
                        }
                        style=move || {
                            if style_host.is_some() {
                                "cursor: pointer;"
                            } else {
                                "opacity: 0.5; cursor: not-allowed;"
                            }
                        }
                    >
                        <td>{entry.ip.clone()}</td>
                        <td>{entry.port}</td>
                        <td>{entry.org.clone()}</td>
                        <td>{entry.city.clone()}</td>
                        <td>{entry.country.clone()}</td>
                        <td>
                            <button on:click=move |ev| {
                                ev.stop_propagation();
                                log::info!("Ping {}", ip_ping);
                            }>"Ping"</button>
                        </td>
                        <td>
                            <button on:click=move |ev| {
                                ev.stop_propagation();
                                log::info!("Try default credentials on {}", ip_creds);
                            }>"Try Default Credentials"</button>
                        </td>
                    </tr>
                }
            }
        />
    }
}
