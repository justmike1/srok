#![cfg(feature = "ssr")]

use axum::http::HeaderValue;
use tower_http::cors::{Any, CorsLayer};

use crate::utils::config::get_origin_base_url;

// TODO: CSRF protection

pub fn cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(
            HeaderValue::from_str(get_origin_base_url()).expect("Invalid ORIGIN_BASE_URL"),
        )
        .allow_methods(Any)
        .allow_headers(Any)
}
