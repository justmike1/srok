#![cfg(feature = "ssr")]

use axum::{
    body::Body,
    http::{HeaderValue, Request, StatusCode},
    middleware::Next,
    response::Response,
};
use tower_http::cors::{Any, CorsLayer};

use crate::utils::config::{get_frontend_secret, get_origin_base_url};

pub async fn enforce_web_guard(req: Request<Body>, next: Next) -> Result<Response, StatusCode> {
    let origin = req
        .headers()
        .get("origin")
        .and_then(|v| v.to_str().ok())
        .ok_or(StatusCode::FORBIDDEN)?;

    if origin != get_origin_base_url() {
        return Err(StatusCode::FORBIDDEN);
    }

    let secret = req
        .headers()
        .get("x-frontend-secret")
        .and_then(|v| v.to_str().ok())
        .ok_or(StatusCode::FORBIDDEN)?;

    if secret != get_frontend_secret() {
        return Err(StatusCode::FORBIDDEN);
    }

    Ok(next.run(req).await)
}

pub fn cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(
            HeaderValue::from_str(get_origin_base_url()).expect("Invalid ORIGIN_BASE_URL"),
        )
        .allow_methods(Any)
        .allow_headers(Any)
}
