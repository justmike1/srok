use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResultRO<T> {
    pub success: bool,
    pub error: Option<String>,
    pub error_object: Option<ResultErrorDTO>,
    pub result: Option<T>,
    pub paging: Option<PagingRO>,
}

impl<T> Default for ResultRO<T> {
    fn default() -> Self {
        Self {
            success: true,
            error: None,
            error_object: None,
            result: None,
            paging: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct PagingRO {
    pub start: Option<usize>,
    pub limit: Option<usize>,
    pub total: Option<u64>,
    pub has_more: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResultErrorDTO {
    pub code: Option<String>,
    pub message: Option<String>,
}
