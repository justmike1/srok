#[cfg(feature = "ssr")]
use axum::http::StatusCode;
#[cfg(feature = "ssr")]
use serde::Deserialize;
#[cfg(feature = "ssr")]
use std::error::Error;
#[cfg(feature = "ssr")]
use std::fmt;

#[cfg(feature = "ssr")]
#[derive(Debug, Deserialize)]
pub struct GithubParams {
    pub term: String,
}

#[cfg(feature = "ssr")]
#[derive(Debug)]
pub struct GithubError {
    pub message: String,
    pub status: StatusCode,
}

#[cfg(feature = "ssr")]
impl fmt::Display for GithubError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

#[cfg(feature = "ssr")]
impl Error for GithubError {}
