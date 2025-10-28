use crate::{Columns, Row, RowData, TableContext, TableData};
use dioxus::prelude::*;

pub fn use_tabular<C: Columns<R>, R: Row>(
    columns: C,
    rows: ReadOnlySignal<Vec<R>>,
) -> TableData<C, R> {
    let context = TableContext::use_table_context(columns);
    context.table_data(rows)
}

#[component]
pub fn TableHeaders<C: Columns<R>, R: Row>(
    data: TableData<C, R>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        for header in data.context.headers() {
            Fragment { key: "{header.key()}", {header.render(attributes.clone())} }
        }
    }
}

#[component]
pub fn TableCells<C: Columns<R>, R: Row>(
    row: RowData<C, R>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        for cell in row.cells() {
            Fragment { key: "{cell.key()}", {cell.render(attributes.clone())} }
        }
    }
}
