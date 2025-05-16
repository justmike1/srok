#![cfg(feature = "ssr")]

use axum::{
    body::Body,
    http::{HeaderValue, Request, StatusCode},
    middleware::Next,
    response::Response,
};
use log::debug;
use tower_http::cors::{Any, CorsLayer};

use crate::utils::config::{get_frontend_secret, get_origin_base_url};

/// Middleware to enforce CORS Origin and `X-Frontend-Secret` for API routes
pub async fn enforce_web_guard<B>(req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    let origin_ok = req
        .headers()
        .get("origin")
        .and_then(|v| v.to_str().ok())
        .map(|o| o == get_origin_base_url())
        .unwrap_or(false);

    let secret_ok = req
        .headers()
        .get("x-frontend-secret")
        .and_then(|v| v.to_str().ok())
        .map(|s| s == get_frontend_secret())
        .unwrap_or(false);

    if origin_ok && secret_ok {
        Ok(next.run(req).await)
    } else {
        debug!(
            "Blocked request: Origin OK: {}, Secret OK: {}",
            origin_ok, secret_ok
        );
        Err(StatusCode::FORBIDDEN)
    }
}

/// Reusable CORS layer restricted to configured frontend origin
pub fn cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(
            HeaderValue::from_str(get_origin_base_url()).expect("Invalid ORIGIN_BASE_URL"),
        )
        .allow_methods(Any)
        .allow_headers(Any)
}
