use crate::{Columns, Row, TableContext};
use dioxus::prelude::*;

pub fn use_tabular<C: Columns<R>, R: Row>(
    columns: C,
    rows: ReadOnlySignal<Vec<R>>,
) -> TableData<C, R> {
    let context = TableContext::use_table_context(columns);
    TableData { context, rows }
}

#[derive(PartialEq)]
pub struct TableData<C: Columns<R>, R: Row> {
    pub context: TableContext<C>,
    pub rows: ReadOnlySignal<Vec<R>>,
}

impl<C: Columns<R>, R: Row> Clone for TableData<C, R> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<C: Columns<R>, R: Row> Copy for TableData<C, R> {}

impl<C: Columns<R>, R: Row> TableData<C, R> {
    pub fn rows(&self) -> impl Iterator<Item = RowData<C, R>> {
        // TODO: apply filters and sorts
        self.rows.iter().enumerate().map(|(i, _row)| RowData {
            context: self.context,
            rows: self.rows,
            index: i,
            _phantom: std::marker::PhantomData,
        })
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct RowData<C: Columns<R>, R: Row> {
    context: TableContext<C>,
    rows: ReadOnlySignal<Vec<R>>,
    index: usize,
    _phantom: std::marker::PhantomData<R>,
}

impl<C: Columns<R>, R: Row> RowData<C, R> {
    pub fn key(&self) -> String {
        self.rows.read()[self.index].key().into()
    }
}

#[component]
pub fn TableHeaders<C: Columns<R>, R: Row>(
    data: TableData<C, R>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        for (name , render) in data.context
            .columns
            .read()
            .column_names()
            .into_iter()
            .zip(data.context.columns.read().headers())
        {
            Fragment { key: "{name}", {render(&data.context, attributes.clone())} }
        }
    }
}

#[component]
pub fn TableCells<C: Columns<R>, R: Row>(
    row: RowData<C, R>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        for (name , render) in row.context
            .columns
            .read()
            .column_names()
            .into_iter()
            .zip(row.context.columns.read().columns())
        {
            Fragment { key: "{name}",
                {render(&row.context, &row.rows.read()[row.index], attributes.clone())}
            }
        }
    }
}
