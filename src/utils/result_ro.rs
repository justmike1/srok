use crate::services::ros::{PagingRO, ResultRO};
use crate::utils::config::ROWS_PER_PAGE;
use serde::Serialize;
use serde_json::Value;

pub fn to_result_ro<T>(
    data: &T,
    page: usize,
    count: usize,
    total: usize,
) -> Result<ResultRO<Value>, String>
where
    T: Serialize,
{
    let rows_per_page = ROWS_PER_PAGE;
    let start = if page == 0 {
        0
    } else {
        (page - 1) * rows_per_page
    };
    let has_more = start + count < total;

    let paging = PagingRO {
        start: Some(start),
        limit: Some(start + count),
        total: Some(total as u64),
        has_more: Some(has_more),
    };

    let result_json = serde_json::to_value(data).map_err(|e| e.to_string())?;

    Ok(ResultRO {
        result: Some(result_json),
        paging: Some(paging),
        ..Default::default()
    })
}
