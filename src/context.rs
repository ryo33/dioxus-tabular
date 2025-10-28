use dioxus::prelude::*;

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
pub struct TableContext {
    sorts: Signal<Vec<SortRecord>>,
    // The columns names of the table.
    columns: Signal<Vec<String>>,
    // If exists, the order of the columns is overridden by this order. It's ok to hide some columns, but must not have duplicates.
    override_order: Signal<Option<Vec<usize>>>,
}

impl TableContext {
    pub fn use_table_context(column_names: Vec<String>) -> Self {
        let sorts = use_signal(Vec::new);
        debug_assert!(
            column_names.len()
                == column_names
                    .iter()
                    .collect::<std::collections::HashSet<_>>()
                    .len(),
            "Column names must be unique"
        );
        let columns = use_signal(|| column_names);
        let override_order = use_signal(|| None);
        Self {
            sorts,
            columns,
            override_order,
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

    pub fn column_context(&self, column: usize) -> ColumnContext {
        ColumnContext {
            table_context: *self,
            column,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct ColumnContext {
    table_context: TableContext,
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
