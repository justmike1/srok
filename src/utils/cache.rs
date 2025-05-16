#![cfg(feature = "ssr")]

use std::sync::OnceLock;

use log::debug;
use quick_cache::sync::Cache;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;

/// Lazy-initialized JSON cache with a 1-hour TTL
pub fn get_or_init_cache(
    tag: &'static OnceLock<Cache<String, Value>>,
) -> &'static Cache<String, Value> {
    tag.get_or_init(|| Cache::new(3600))
}

pub fn try_get_from_cache<T: DeserializeOwned>(
    cache: &Cache<String, Value>,
    key: &str,
) -> Option<T> {
    cache.get(key).and_then(|value| {
        serde_json::from_value::<T>(value.clone())
            .map_err(|e| debug!("Failed to deserialize cached value for {key}: {e}"))
            .ok()
    })
}

pub fn insert_into_cache<T: Serialize>(cache: &Cache<String, Value>, key: &str, value: &T) {
    if let Ok(json) = serde_json::to_value(value) {
        cache.insert(key.to_string(), json);
    } else {
        debug!("Failed to serialize value for cache key: {key}");
    }
}
