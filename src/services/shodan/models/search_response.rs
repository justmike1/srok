use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ShodanSearchResponse {
    pub matches: Vec<HostMatch>,
    pub total: u64,
    pub facets: Option<std::collections::HashMap<String, Vec<Facet>>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Facet {
    pub count: u64,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HostMatch {
    pub ip_str: String,
    pub port: Option<u16>,
    pub org: Option<String>,
    pub location: Option<Location>,
    pub http: Option<HttpInfo>,
    pub hostnames: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    pub city: Option<String>,
    pub country_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpInfo {
    pub host: Option<String>,
    pub title: Option<String>,
}
