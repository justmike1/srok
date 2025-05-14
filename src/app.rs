use crate::components::{IntegrationButton, IntegrationPage};
use crate::integrations::Integration;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    path, StaticSegment,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum HomeView {
    Tools,
    Secrets,
    Wifi,
}

impl Default for HomeView {
    fn default() -> Self {
        HomeView::Tools
    }
}

impl ToString for HomeView {
    fn to_string(&self) -> String {
        match self {
            HomeView::Tools => "TOOLS".to_string(),
            HomeView::Secrets => "SECRETS".to_string(),
            HomeView::Wifi => "WIFI".to_string(),
        }
    }
}

impl std::str::FromStr for HomeView {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "SECRETS" => Ok(HomeView::Secrets),
            "TOOLS" => Ok(HomeView::Tools),
            "WIFI" => Ok(HomeView::Wifi),
            _ => Err(()),
        }
    }
}

pub fn shell(options: LeptosOptions) -> impl IntoView {
    let options_clone = options.clone();

    provide_meta_context();

    view! {
        <!DOCTYPE html>
        <html lang="en" data-theme="bw">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <Title text="Srok – Surveillance Rootkit for Open Knowledge"/>
                <MetaTags />
                <HydrationScripts options />
                <AutoReload options=options_clone />
                <Stylesheet id="leptos" href="/pkg/srok.css" />
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn app() -> impl IntoView {
    provide_meta_context();

    view! {
        <Router>
            <main class="container">
                <Routes fallback=|| view! { <p class="not-found">"404 – Page not found."</p> }>
                    <Route path=StaticSegment("") view=HomePage/>
                    <Route path=path!("integration/:tool") view=IntegrationPage/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn home_page() -> impl IntoView {
    let (toggle_state, set_toggle_state) = signal(HomeView::default());
    let (show_modal, set_show_modal) = signal(true);

    Effect::new({
        let set_show_modal = set_show_modal.clone();
        move |_| {
            if let Some(win) = web_sys::window() {
                match win.local_storage() {
                    Ok(Some(storage)) => {
                        if let Ok(Some(agreed)) = storage.get_item("srok_disclaimer_agreed") {
                            if agreed == "true" {
                                set_show_modal.set(false);
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    });

    let agree = move |_| {
        if let Some(win) = web_sys::window() {
            if let Ok(Some(storage)) = win.local_storage() {
                let _ = storage.set_item("srok_disclaimer_agreed", "true");
            }
        }
        set_show_modal.set(false);
    };

    let integrations = Memo::new({
        let toggle_state = toggle_state.clone();
        move |_| {
            let filter = match toggle_state.get() {
                HomeView::Tools => Integration::is_tool,
                HomeView::Secrets => Integration::is_secret,
                HomeView::Wifi => Integration::is_wifi,
            };
            Integration::all()
                .into_iter()
                .filter(|i| filter(i))
                .collect::<Vec<_>>()
        }
    });

    view! {
        <>
            {move || show_modal.get().then(|| view! {
                <div class="modal-backdrop">
                    <div class="modal">
                        <h2>"Legal Disclaimer"</h2>
                        <p>
                            "This tool is for authorized and educational use only. Unauthorized access, testing, or scanning may violate applicable laws."
                        </p>
                        <p>
                            "By clicking 'I Agree', you acknowledge that you have read and understood the disclaimer, and accept full responsibility for any use."
                        </p>
                        <button on:click=agree class="agree-button">"I Agree"</button>
                    </div>
                </div>
            })}

            <section class="homepage" style:filter=move || if show_modal.get() { "blur(4px)" } else { "none" }>
                <header>
                    <h1 class="logo">Srok</h1>
                    <p class="tagline">System-wide Reconnaissance & Observation Kernel</p>
                </header>

                <div class="toggle-wrapper">
                    {[HomeView::Tools, HomeView::Secrets, HomeView::Wifi].into_iter().map(|view| {
                        let view_str = view.to_string();
                        let is_active = {
                            let toggle_state = toggle_state.clone();
                            move || toggle_state.get() == view
                        };
                        let set_view = {
                            let set_toggle_state = set_toggle_state.clone();
                            let view = view.clone();
                            move |_| set_toggle_state.set(view.clone())
                        };

                        view! {
                            <button
                                class=move || {
                                    let mut class = "toggle-button".to_string();
                                    if is_active() {
                                        class.push_str(" active");
                                    }
                                    class
                                }
                                on:click=set_view
                            >
                                {view_str}
                            </button>
                        }
                    }).collect_view()}
                </div>

                <div class="integrations">
                    {move || integrations.get().iter().map(|tool| {
                        let name = Box::leak(tool.to_string().into_boxed_str());
                        let logo = Box::leak(
                            format!(
                                "/logos/{}.svg",
                                tool.to_string().to_lowercase().replace(' ', "")
                            ).into_boxed_str(),
                        );
                        view! {
                            <IntegrationButton name=name logo=logo />
                        }
                    }).collect_view()}
                </div>

                <footer class="disclaimer">
                    <p>
                        <strong>Disclaimer:</strong> This tool is intended for authorized testing and educational use only. Unauthorized use is strictly prohibited.
                        The author assumes no liability for misuse or damage caused by this software. Use responsibly and comply with all applicable laws.
                    </p>
                </footer>
            </section>
        </>
    }
}
