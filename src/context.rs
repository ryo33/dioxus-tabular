use dioxus::prelude::*;

use crate::{Columns, Row};
use std::marker::PhantomData;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum SortDirection {
    Ascending,
    Descending,
}

#[derive(Clone, Copy, PartialEq)]
pub struct Sort {
    pub direction: SortDirection,
}

#[derive(Clone, Copy, PartialEq)]
pub enum SortGesture {
    Cancel,
    AddFirst(Sort),
    AddLast(Sort),
}

#[derive(Clone, Copy, PartialEq)]
pub struct SortRecord {
    column: usize,
    sort: Sort,
}

#[derive(Clone, Copy, PartialEq)]
pub(crate) struct TableContextData {
    sorts: Signal<Vec<SortRecord>>,
    // The columns names of the table.
    column_names: Signal<Vec<String>>,
    // If exists, the order of the columns is overridden by this order. It's ok to hide some columns, but must not have duplicates.
    override_order: Signal<Option<Vec<usize>>>,
}

#[derive(PartialEq)]
pub struct TableContext<C: 'static> {
    pub(crate) data: TableContextData,
    /// Does not need to be Signal, but for Copy trait.
    pub(crate) columns: Signal<C>,
}

impl<C: 'static> Copy for TableContext<C> {}

impl<C: 'static> Clone for TableContext<C> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<C> TableContext<C> {
    pub fn use_table_context<R>(columns: C) -> Self
    where
        C: Columns<R>,
        R: Row,
    {
        let sorts = use_signal(Vec::new);
        let column_names = use_signal(|| columns.column_names());
        let override_order = use_signal(|| None);
        let columns = use_signal(|| columns);
        Self {
            data: TableContextData {
                sorts,
                column_names,
                override_order,
            },
            columns,
        }
    }

    pub fn table_data<R>(self, rows: ReadOnlySignal<Vec<R>>) -> TableData<C, R>
    where
        C: Columns<R>,
        R: Row,
    {
        TableData {
            context: self,
            rows,
        }
    }

    fn get_column_order(&self) -> Vec<usize> {
        self.data.get_column_order()
    }

    pub fn headers<R>(self) -> impl Iterator<Item = HeaderData<C, R>>
    where
        C: Columns<R>,
        R: Row,
    {
        let order = self.get_column_order();
        order.into_iter().map(move |column_index| HeaderData {
            context: self,
            column_index,
            _phantom: PhantomData,
        })
    }

    pub fn cells<R>(self, row: RowData<C, R>) -> impl Iterator<Item = CellData<C, R>>
    where
        C: Columns<R>,
        R: Row,
    {
        let order = self.get_column_order();
        let row_copy = row;
        order.into_iter().map(move |column_index| CellData {
            row: row_copy,
            column_index,
        })
    }

    pub fn rows<R>(self, rows: ReadOnlySignal<Vec<R>>) -> impl Iterator<Item = RowData<C, R>>
    where
        C: Columns<R>,
        R: Row,
    {
        // TODO: apply filters and sorts
        let len = rows.read().len();
        (0..len).map(move |i| RowData {
            context: self,
            rows,
            index: i,
            _phantom: PhantomData,
        })
    }
}

impl TableContextData {
    pub fn column_context(&self, column: usize) -> ColumnContext {
        ColumnContext {
            table_context: *self,
            column,
        }
    }

    pub fn get_column_order(&self) -> Vec<usize> {
        if let Some(order) = self.override_order.read().as_ref() {
            order.clone()
        } else {
            (0..self.column_names.read().len()).collect()
        }
    }

    pub fn get_column_name(&self, index: usize) -> String {
        self.column_names.read()[index].clone()
    }

    pub fn request_sort(&self, column: usize, sort: SortGesture) {
        match sort {
            SortGesture::Cancel => {
                let mut signal = self.sorts;
                signal.write().retain(|record| record.column != column);
            }
            SortGesture::AddFirst(sort) => {
                let mut signal = self.sorts;
                let mut write = signal.write();
                write.retain(|record| record.column != column);
                write.insert(0, SortRecord { column, sort });
            }
            SortGesture::AddLast(sort) => {
                let mut signal = self.sorts;
                let mut write = signal.write();
                write.retain(|record| record.column != column);
                write.push(SortRecord { column, sort });
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct ColumnContext {
    table_context: TableContextData,
    column: usize,
}

impl ColumnContext {
    pub fn request_sort(&self, sort: SortGesture) {
        self.table_context.request_sort(self.column, sort);
    }

    /// Returns the position of the sort in the list of sorts, or None if no sort is applied to this column.
    pub fn sort_number(&self) -> Option<usize> {
        self.table_context
            .sorts
            .read()
            .iter()
            .position(|record| record.column == self.column)
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct HeaderData<C: Columns<R>, R: Row> {
    context: TableContext<C>,
    column_index: usize,
    _phantom: PhantomData<R>,
}

impl<C: Columns<R>, R: Row> HeaderData<C, R> {
    pub fn key(&self) -> String {
        self.context.data.get_column_name(self.column_index)
    }

    pub fn render(&self, attributes: Vec<Attribute>) -> Element {
        let binding = self.context.columns.read();
        let headers = binding.headers();
        headers[self.column_index](&self.context, attributes)
    }
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
        self.context.rows(self.rows)
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct CellData<C: Columns<R>, R: Row> {
    row: RowData<C, R>,
    column_index: usize,
}

impl<C: Columns<R>, R: Row> CellData<C, R> {
    pub fn key(&self) -> String {
        self.row.context.data.get_column_name(self.column_index)
    }

    pub fn render(&self, attributes: Vec<Attribute>) -> Element {
        let binding = self.row.context.columns.read();
        let columns = binding.columns();
        columns[self.column_index](
            &self.row.context,
            &self.row.rows.read()[self.row.index],
            attributes,
        )
    }
}

#[derive(PartialEq)]
pub struct RowData<C: Columns<R>, R: Row> {
    pub(crate) context: TableContext<C>,
    pub(crate) rows: ReadOnlySignal<Vec<R>>,
    pub(crate) index: usize,
    pub(crate) _phantom: PhantomData<R>,
}

impl<C: Columns<R>, R: Row> Copy for RowData<C, R> {}

impl<C: Columns<R>, R: Row> Clone for RowData<C, R> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<C: Columns<R>, R: Row> RowData<C, R> {
    pub fn key(&self) -> String {
        self.rows.read()[self.index].key().into()
    }

    pub fn cells(self) -> impl Iterator<Item = CellData<C, R>> {
        self.context.cells(self)
    }
}

#[cfg(test)]
mod tests_sort_request;
