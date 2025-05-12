use leptos::prelude::*;
use std::sync::Arc;

#[component]
pub fn PagedTable<T, FHeader, FRow, IVH, IVR>(
    entries: Arc<Vec<T>>,
    header: FHeader,
    row: FRow,
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

    let total_items = entries.len();
    let total_pages = move || (total_items + rows_per_page - 1) / rows_per_page;

    let paged_entries = move || {
        if total_items == 0 {
            Vec::new()
        } else {
            let start = page() * rows_per_page;
            let end = usize::min(start + rows_per_page, total_items);
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
                        if total_items == 0 {
                            "0 items".to_string()
                        } else {
                            format!(
                                "{}–{} of {} items",
                                page() * rows_per_page + 1,
                                usize::min((page() + 1) * rows_per_page, total_items),
                                total_items
                            )
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
                    disabled={ move || total_items == 0 || page() + 1 >= total_pages() }
                >
                    {">"}
                </button>
            </div>
        </div>
    }
}
