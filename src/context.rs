use dioxus::prelude::*;

use crate::{Columns, Row};

#[derive(Clone, Copy, PartialEq)]
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
}

impl TableContextData {
    pub fn column_context(&self, column: usize) -> ColumnContext {
        ColumnContext {
            table_context: *self,
            column,
        }
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
