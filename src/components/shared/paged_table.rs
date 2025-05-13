use leptos::prelude::*;
use std::sync::Arc;

use crate::services::ros::PagingRO;

#[component]
pub fn PagedTable<T, FHeader, FRow, IVH, IVR>(
    entries: Arc<Vec<T>>,
    header: FHeader,
    row: FRow,
    paging: PagingRO,
    page: ReadSignal<usize>,
    set_page: WriteSignal<usize>,
    on_page_change: Callback<usize>,
    #[prop(optional, default = 50)] rows_per_page: usize,
) -> impl IntoView
where
    T: Clone + Send + Sync + 'static,
    FHeader: Fn() -> IVH + 'static,
    IVH: IntoView + 'static,
    FRow: Fn(&T) -> IVR + Send + Sync + 'static,
    IVR: IntoView + 'static,
{
    let total = paging.total.unwrap_or(0) as usize;
    let has_more = paging.has_more.unwrap_or(false);

    view! {
        <div class="table-container">
            <div class="table-scroll">
                <table class="leptos-datatable">
                    <thead>{ header().into_view() }</thead>
                    <tbody>
                        {
                            entries.iter()
                                .map(|entry| row(entry).into_view())
                                .collect::<Vec<_>>()
                        }
                    </tbody>
                </table>
            </div>
            <div class="pagination-controls">
                <span>
                    { move || {
                        if entries.is_empty() {
                            "0 items".to_string()
                        } else {
                            let start = page.get() * rows_per_page + 1;
                            let end = usize::min((page.get() + 1) * rows_per_page, total);
                            format!("{start}–{end} of {total} items")
                        }
                    }}
                </span>
                <button
                    on:click=move |_| {
                        let prev = page.get().saturating_sub(1);
                        set_page.set(prev);
                        on_page_change.run(prev);
                    }
                    disabled=move || page.get() == 0
                >
                    {"<"}
                </button>
                <button
                    on:click=move |_| {
                        let next = page.get() + 1;
                        set_page.set(next);
                        on_page_change.run(next);
                    }
                    disabled=move || !has_more
                >
                    {">"}
                </button>
            </div>
        </div>
    }
}
