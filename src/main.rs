use axum::{routing::get, Router};
use dotenv::dotenv;
use leptos::config::get_configuration;
use leptos_axum::{generate_route_list, handle_server_fns_with_context, LeptosRoutes};
use log::info;
use srok::app::{shell, App};
use std::net::SocketAddr;

async fn healthcheck() -> &'static str {
    "ok"
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    info!("Starting Srok server...");

    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    let addr: SocketAddr = leptos_options.site_addr.parse().unwrap();

    let options_for_leptos_routes = leptos_options.clone();
    let options_for_state = leptos_options.clone();

    let app = Router::new()
        .route("/healthz", get(healthcheck))
        .route(
            "/api/*fn_name",
            get(move |req| async move { handle_server_fns_with_context(|| (), req).await }),
        )
        .leptos_routes(
            &options_for_leptos_routes,
            generate_route_list(App),
            move || shell(options_for_leptos_routes.clone()),
        )
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(options_for_state);

    info!("Listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
