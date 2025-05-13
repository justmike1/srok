use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ShodanSearchResponse {
    pub matches: Vec<HostMatch>,
    pub total: u64,
    pub facets: Option<HashMap<String, Vec<Facet>>>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Facet {
    pub count: u64,
    pub value: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct HostMatch {
    pub ip_str: String,
    pub port: Option<u16>,
    pub org: Option<String>,
    pub location: Option<Location>,
    pub http: Option<HttpInfo>,
    pub hostnames: Option<Vec<String>>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Location {
    pub city: Option<String>,
    pub country_name: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct HttpInfo {
    pub host: Option<String>,
    pub title: Option<String>,
}
