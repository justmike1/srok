use leptos::prelude::*;
use std::sync::Arc;

#[component]
pub fn PagedTable<T, FHeader, FRow, IVH, IVR>(
    entries: Arc<Vec<T>>,
    header: FHeader,
    row: FRow,
    #[prop(optional)] total: Option<usize>,
    #[prop(optional, default = 50)] rows_per_page: usize,
) -> impl IntoView
where
    T: Clone + Send + Sync + 'static,
    FHeader: Fn() -> IVH + 'static,
    IVH: IntoView + 'static,
    FRow: Fn(&T) -> IVR + Send + Sync + 'static,
    IVR: IntoView + 'static,
{
    let (page, set_page) = signal(0usize);

    let local_count = entries.len();
    let total_count = total.unwrap_or(local_count);

    let total_pages = move || (local_count + rows_per_page - 1) / rows_per_page;

    let paged_entries = move || {
        if local_count == 0 {
            Vec::new()
        } else {
            let start = page() * rows_per_page;
            let end = usize::min(start + rows_per_page, local_count);
            entries[start..end].to_vec()
        }
    };

    view! {
        <div class="table-container">
            <div class="table-scroll">
                <table class="leptos-datatable">
                    <thead>{ header().into_view() }</thead>
                    <tbody>
                        { move || {
                            paged_entries()
                                .into_iter()
                                .map(|entry| row(&entry).into_view())
                                .collect::<Vec<_>>()
                        }}
                    </tbody>
                </table>
            </div>
            <div class="pagination-controls">
                <span>
                    { move || {
                        if local_count == 0 {
                            "0 items".to_string()
                        } else {
                            let start = page() * rows_per_page + 1;
                            let end = usize::min((page() + 1) * rows_per_page, total_count);
                            format!("{start}–{end} of {total_count} items")
                        }
                    }}
                </span>
                <button
                    on:click=move |_| set_page(page().saturating_sub(1))
                    disabled={ move || page() == 0 }
                >
                    {"<"}
                </button>
                <button
                    on:click=move |_| {
                        let tp = total_pages();
                        if tp > 0 {
                            set_page(usize::min(page() + 1, tp.saturating_sub(1)))
                        }
                    }
                    disabled={ move || local_count == 0 || page() + 1 >= total_pages() }
                >
                    {">"}
                </button>
            </div>
        </div>
    }
}
