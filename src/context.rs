use dioxus::prelude::*;

use crate::{Columns, Row};
use std::marker::PhantomData;

mod column_order;
pub use column_order::ColumnOrder;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum SortDirection {
    Ascending,
    Descending,
}

#[derive(Clone, Copy, PartialEq)]
pub struct Sort {
    pub direction: SortDirection,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct SortInfo {
    pub priority: usize,
    pub direction: SortDirection,
}

#[derive(Clone, Copy, PartialEq)]
pub enum SortGesture {
    Cancel,
    AddFirst(Sort),
    AddLast(Sort),
    Toggle,
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
    // Manages the order and visibility of columns.
    column_order: Signal<ColumnOrder>,
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
        let total_columns = column_names.read().len();
        let column_order = use_signal(|| ColumnOrder::new(total_columns));
        let columns = use_signal(|| columns);
        Self {
            data: TableContextData {
                sorts,
                column_names,
                column_order,
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
        let rows_data = rows.read();
        let columns = self.columns.read();

        // Step 1: Apply filter - collect indices of rows that pass the filter
        let mut filtered_indices: Vec<usize> = (0..rows_data.len())
            .filter(|&i| columns.filter(&rows_data[i]))
            .collect();

        // Step 2: Apply sort if any sort records exist
        let sort_records = self.data.sorts.read();
        if !sort_records.is_empty() {
            let comparators = columns.compare();

            // Sort the filtered indices based on multi-column sort priority
            filtered_indices.sort_by(|&a, &b| {
                // Iterate through sort records in priority order
                for sort_record in sort_records.iter() {
                    let ordering = comparators[sort_record.column](&rows_data[a], &rows_data[b]);

                    // Apply direction (ascending or descending)
                    let directed_ordering = match sort_record.sort.direction {
                        SortDirection::Ascending => ordering,
                        SortDirection::Descending => ordering.reverse(),
                    };

                    // If not equal, return this ordering
                    if directed_ordering != std::cmp::Ordering::Equal {
                        return directed_ordering;
                    }
                    // If equal, continue to next sort column
                }

                // All sort columns are equal, maintain stable sort
                std::cmp::Ordering::Equal
            });
        }

        // Step 3: Return iterator over sorted and filtered indices
        filtered_indices.into_iter().map(move |i| RowData {
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
        self.column_order.read().get_order().to_vec()
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
            SortGesture::Toggle => {
                let mut signal = self.sorts;
                if let Some(record) = signal.write().iter_mut().find(|r| r.column == column) {
                    record.sort.direction = match record.sort.direction {
                        SortDirection::Ascending => SortDirection::Descending,
                        SortDirection::Descending => SortDirection::Ascending,
                    };
                }
            }
        }
    }

    // Column order management methods

    pub fn swap_columns(&self, col_a: usize, col_b: usize) {
        let mut signal = self.column_order;
        signal.write().swap(col_a, col_b);
    }

    pub fn hide_column(&self, col: usize) {
        let mut signal = self.column_order;
        signal.write().hide_column(col);
    }

    pub fn show_column(&self, col: usize, at_index: Option<usize>) {
        let mut signal = self.column_order;
        signal.write().show_column(col, at_index);
    }

    pub fn move_column_to(&self, col: usize, new_index: usize) {
        let mut signal = self.column_order;
        signal.write().move_to(col, new_index);
    }

    pub fn move_column_forward(&self, col: usize) {
        let mut signal = self.column_order;
        signal.write().move_forward(col);
    }

    pub fn move_column_backward(&self, col: usize) {
        let mut signal = self.column_order;
        signal.write().move_backward(col);
    }

    pub fn is_column_visible(&self, col: usize) -> bool {
        self.column_order.read().is_visible(col)
    }

    pub fn column_position(&self, col: usize) -> Option<usize> {
        self.column_order.read().position(col)
    }

    pub fn reset_column_order(&self) {
        let mut signal = self.column_order;
        signal.write().reset();
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

    /// Returns the sort information (priority and direction) for this column, or None if not sorted.
    pub fn sort_info(&self) -> Option<SortInfo> {
        let sorts = self.table_context.sorts.read();
        sorts
            .iter()
            .position(|record| record.column == self.column)
            .map(|priority| SortInfo {
                priority,
                direction: sorts[priority].sort.direction,
            })
    }

    // Column order management delegate methods

    /// Swaps this column with another column in the display order.
    pub fn swap_with(&self, other_col: usize) {
        self.table_context.swap_columns(self.column, other_col);
    }

    /// Hides this column from the display.
    pub fn hide(&self) {
        self.table_context.hide_column(self.column);
    }

    /// Shows this column in the display.
    /// If at_index is None, the column is appended to the end.
    pub fn show(&self, at_index: Option<usize>) {
        self.table_context.show_column(self.column, at_index);
    }

    /// Moves this column to a specific position in the display order (0-indexed).
    pub fn move_to(&self, new_index: usize) {
        self.table_context.move_column_to(self.column, new_index);
    }

    /// Moves this column one position forward (towards index 0).
    pub fn move_forward(&self) {
        self.table_context.move_column_forward(self.column);
    }

    /// Moves this column one position backward (towards the end).
    pub fn move_backward(&self) {
        self.table_context.move_column_backward(self.column);
    }

    /// Returns whether this column is currently visible.
    pub fn is_visible(&self) -> bool {
        self.table_context.is_column_visible(self.column)
    }

    /// Returns the display position of this column (0-indexed), or None if hidden.
    pub fn position(&self) -> Option<usize> {
        self.table_context.column_position(self.column)
    }

    /// Resets the column order to the default state (all columns visible in natural order).
    pub fn reset_order(&self) {
        self.table_context.reset_column_order();
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct HeaderData<C: Columns<R>, R: Row> {
    pub(crate) context: TableContext<C>,
    pub(crate) column_index: usize,
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
    pub(crate) row: RowData<C, R>,
    pub(crate) column_index: usize,
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

#[cfg(test)]
mod tests_rows_filter_and_sort;
