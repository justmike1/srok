#[cfg(feature = "ssr")]
use serde::Deserialize;

#[cfg(feature = "ssr")]
#[derive(Debug, Deserialize)]
pub struct ShodanParams {
    pub tool: String,
}
