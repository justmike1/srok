use crate::components::{IntegrationButton, IntegrationPage};
use crate::integrations::Integration;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    path, StaticSegment,
};

#[derive(Clone, PartialEq, Eq, Debug)]
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
pub fn App() -> impl IntoView {
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
fn HomePage() -> impl IntoView {
    let (toggle_state, set_toggle_state) = signal(HomeView::default());

    let integrations = Memo::new(move |_| {
        let filter = match toggle_state.get() {
            HomeView::Tools => Integration::is_tool,
            HomeView::Secrets => Integration::is_secret,
            HomeView::Wifi => Integration::is_wifi,
        };
        Integration::all()
            .into_iter()
            .filter(|i| filter(i))
            .collect::<Vec<_>>()
    });

    view! {
        <section class="homepage">
            <header>
                <h1 class="logo">Srok</h1>
                <p class="tagline">System-wide Reconnaissance & Observation Kernel</p>
            </header>

            <div class="toggle-wrapper">
                {[HomeView::Tools, HomeView::Secrets, HomeView::Wifi].into_iter().map(|view| {
                    let view_str = view.to_string();
                    let view_for_active = view.clone();
                    let view_for_set = view.clone();
                    let is_active = move || toggle_state.get() == view_for_active.clone();
                    let set_view = move |_| set_toggle_state.set(view_for_set.clone());

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
    }
}
