use crate::{
    CellData, Columns, HeaderData, Row, SerializableColumns, TableColumn, TableContext, TableData,
};
use dioxus::prelude::*;
use serde::Serialize;

/// A column that can be serialized to various export formats.
///
/// This trait extends [`TableColumn`] with serialization capabilities.
/// Implement this trait when you want to export table data (e.g., to CSV, JSON, Excel).
///
/// # Example
///
/// ```
/// # use dioxus::prelude::*;
/// # use dioxus_tabular::*;
/// # use serde::Serialize;
/// # #[derive(Clone, PartialEq)]
/// # struct User { id: u32, name: String }
/// # impl Row for User {
/// #     fn key(&self) -> impl Into<String> { self.id.to_string() }
/// # }
/// # #[derive(Clone, PartialEq)]
/// # struct UserName(String);
/// # impl GetRowData<UserName> for User {
/// #     fn get(&self) -> UserName { UserName(self.name.clone()) }
/// # }
/// #[derive(Clone, PartialEq)]
/// struct NameColumn;
///
/// impl<R: Row + GetRowData<UserName>> TableColumn<R> for NameColumn {
///     fn column_name(&self) -> String {
///         "name".into()
///     }
///
///     fn render_header(&self, _: ColumnContext, _: Vec<Attribute>) -> Element {
///         rsx! { th { "Name" } }
///     }
///
///     fn render_cell(&self, _: ColumnContext, row: &R, _: Vec<Attribute>) -> Element {
///         rsx! { td { "{row.get().0}" } }
///     }
/// }
///
/// // Enable export by implementing SerializableColumn
/// impl<R: Row + GetRowData<UserName>> SerializableColumn<R> for NameColumn {
///     fn serialize_cell(&self, row: &R) -> impl Serialize + '_ {
///         row.get().0  // Return the name as a string
///     }
/// }
/// ```
pub trait SerializableColumn<R: Row>: TableColumn<R> {
    /// Returns the header text for this column when exported.
    ///
    /// Defaults to [`TableColumn::column_name`], but can be overridden
    /// to provide a different label in exports.
    fn header(&self) -> String {
        self.column_name()
    }

    /// Serializes the cell data for the given row.
    ///
    /// Return any type that implements [`Serialize`]. The exporter will
    /// handle converting it to the appropriate format.
    fn serialize_cell(&self, row: &R) -> impl Serialize + '_;
}

/// Trait for exporting table data to various formats.
///
/// Implement this trait to create custom export formats (CSV, Excel, JSON, etc.).
///
/// # Example
///
/// ```
/// # use dioxus_tabular::Exporter;
/// # use serde::Serialize;
/// struct CsvExporter {
///     output: String,
/// }
///
/// impl Exporter for CsvExporter {
///     type Error = std::fmt::Error;
///
///     fn serialize_header(&self, col: usize, header: &str) -> Result<(), Self::Error> {
///         // Write header to CSV
///         Ok(())
///     }
///
///     fn serialize_cell<'a>(
///         &self,
///         row: usize,
///         col: usize,
///         cell: impl Serialize + 'a,
///     ) -> Result<(), Self::Error> {
///         // Write cell to CSV
///         Ok(())
///     }
/// }
/// ```
pub trait Exporter {
    /// The error type returned by export operations.
    type Error;

    /// Serializes a column header.
    ///
    /// # Parameters
    ///
    /// - `col`: The column index
    /// - `header`: The header text
    fn serialize_header(&mut self, col: usize, header: &str) -> Result<(), Self::Error>;

    /// Serializes a table cell.
    ///
    /// # Parameters
    ///
    /// - `row`: The row index
    /// - `col`: The column index
    /// - `cell`: The serializable cell data
    fn serialize_cell<'a>(
        &mut self,
        row: usize,
        col: usize,
        cell: impl Serialize + 'a,
    ) -> Result<(), Self::Error>;
}

impl<C: Columns<R> + SerializableColumns<R>, R: Row> HeaderData<C, R> {
    fn serialize<E: Exporter>(&self, col: usize, exporter: &mut E) -> Result<(), E::Error> {
        let binding = self.context.columns.read();
        let headers = binding.serialize_headers();
        exporter.serialize_header(col, &headers[self.column_index]())
    }
}

impl<C: Columns<R> + SerializableColumns<R>, R: Row> CellData<C, R> {
    fn serialize<E: Exporter>(
        &self,
        row: usize,
        col: usize,
        exporter: &mut E,
    ) -> Result<(), E::Error> {
        let binding = self.row.context.columns.read();
        let columns = binding.serialize_cell();
        columns[self.column_index](row, col, &self.row.rows.read()[self.row.index], exporter)
    }
}

impl<C: Columns<R> + SerializableColumns<R>, R: Row> TableData<C, R> {
    /// Serializes the table data to the given exporter.
    pub fn serialize<E: Exporter>(&self, exporter: &mut E) -> Result<(), E::Error> {
        self.context.serialize(self.rows, exporter)
    }
}

impl<C> TableContext<C> {
    /// Serializes the table context to the given exporter.
    pub fn serialize<R, E: Exporter>(
        &self,
        rows: ReadSignal<Vec<R>>,
        exporter: &mut E,
    ) -> Result<(), E::Error>
    where
        C: Columns<R> + SerializableColumns<R>,
        R: Row,
    {
        for (col, header) in self.headers().enumerate() {
            header.serialize(col, exporter)?;
        }
        for (row, row_data) in self.rows(rows).enumerate() {
            for (col, cell_data) in row_data.cells().enumerate() {
                cell_data.serialize(row, col, exporter)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_suite::test_hook;
    use crate::{ColumnContext, Sort, SortDirection, SortGesture};

    #[derive(Debug, Clone, PartialEq)]
    struct Person {
        name: String,
        age: u32,
    }

    impl Row for Person {
        fn key(&self) -> impl Into<String> {
            format!("{}_{}", self.name, self.age)
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Serialize)]
    enum Priority {
        High,
    }

    #[derive(Clone, PartialEq)]
    struct NameColumn;
    impl TableColumn<Person> for NameColumn {
        fn column_name(&self) -> String {
            "Name".to_string()
        }
        fn render_header(&self, _context: ColumnContext, _attributes: Vec<Attribute>) -> Element {
            rsx! {
                th {}
            }
        }
        fn render_cell(
            &self,
            _context: ColumnContext,
            _row: &Person,
            _attributes: Vec<Attribute>,
        ) -> Element {
            rsx! {
                td {}
            }
        }
    }
    impl SerializableColumn<Person> for NameColumn {
        fn serialize_cell(&self, row: &Person) -> impl Serialize + '_ {
            row.name.clone()
        }
    }

    #[derive(Clone, PartialEq)]
    struct AgeColumn;
    impl TableColumn<Person> for AgeColumn {
        fn column_name(&self) -> String {
            "Age".to_string()
        }
        fn render_header(&self, _context: ColumnContext, _attributes: Vec<Attribute>) -> Element {
            rsx! {
                th {}
            }
        }
        fn render_cell(
            &self,
            _context: ColumnContext,
            _row: &Person,
            _attributes: Vec<Attribute>,
        ) -> Element {
            rsx! {
                td {}
            }
        }
    }
    impl SerializableColumn<Person> for AgeColumn {
        fn serialize_cell(&self, row: &Person) -> impl Serialize + '_ {
            row.age
        }
    }

    #[derive(Clone, PartialEq)]
    struct PriorityColumn;
    impl TableColumn<Person> for PriorityColumn {
        fn column_name(&self) -> String {
            "Priority".to_string()
        }
        fn render_header(&self, _context: ColumnContext, _attributes: Vec<Attribute>) -> Element {
            rsx! {
                th {}
            }
        }
        fn render_cell(
            &self,
            _context: ColumnContext,
            _row: &Person,
            _attributes: Vec<Attribute>,
        ) -> Element {
            rsx! {
                td {}
            }
        }
    }
    impl SerializableColumn<Person> for PriorityColumn {
        fn header(&self) -> String {
            "Custom Priority Header".to_string()
        }
        fn serialize_cell(&self, _row: &Person) -> impl Serialize + '_ {
            Priority::High
        }
    }

    struct MockExporter {
        headers: Vec<(usize, String)>,
        cells: Vec<(usize, usize, String)>,
    }

    impl MockExporter {
        fn new() -> Self {
            Self {
                headers: Vec::new(),
                cells: Vec::new(),
            }
        }
    }

    impl Exporter for MockExporter {
        type Error = ();

        fn serialize_header(&mut self, col: usize, header: &str) -> Result<(), Self::Error> {
            self.headers.push((col, header.to_string()));
            Ok(())
        }

        fn serialize_cell<'a>(
            &mut self,
            row: usize,
            col: usize,
            cell: impl Serialize + 'a,
        ) -> Result<(), Self::Error> {
            let json = serde_json::to_string(&cell).unwrap();
            self.cells.push((row, col, json));
            Ok(())
        }
    }

    #[test]
    fn test_export_empty_table() {
        test_hook(
            || {
                let context = TableContext::use_table_context((NameColumn, AgeColumn));
                let rows = Signal::new(Vec::<Person>::new());
                (context, rows)
            },
            |(context, rows), _| {
                let mut exporter = MockExporter::new();
                context.serialize(rows.into(), &mut exporter).unwrap();

                assert_eq!(
                    exporter.headers.as_slice(),
                    &[(0, "Name".to_string()), (1, "Age".to_string())]
                );
                assert_eq!(exporter.cells.as_slice(), &[]);
            },
            |_| {},
        );
    }

    #[test]
    fn test_export_multiple_rows_and_columns() {
        test_hook(
            || {
                let context = TableContext::use_table_context((NameColumn, AgeColumn));
                let rows = Signal::new(vec![
                    Person {
                        name: "Alice".to_string(),
                        age: 30,
                    },
                    Person {
                        name: "Bob".to_string(),
                        age: 25,
                    },
                    Person {
                        name: "Charlie".to_string(),
                        age: 35,
                    },
                ]);
                (context, rows)
            },
            |(context, rows), _| {
                let mut exporter = MockExporter::new();
                context.serialize(rows.into(), &mut exporter).unwrap();

                assert_eq!(
                    exporter.headers.as_slice(),
                    &[(0, "Name".to_string()), (1, "Age".to_string())]
                );
                assert_eq!(
                    exporter.cells.as_slice(),
                    &[
                        (0, 0, "\"Alice\"".to_string()),
                        (0, 1, "30".to_string()),
                        (1, 0, "\"Bob\"".to_string()),
                        (1, 1, "25".to_string()),
                        (2, 0, "\"Charlie\"".to_string()),
                        (2, 1, "35".to_string()),
                    ]
                );
            },
            |_| {},
        );
    }

    #[test]
    fn test_export_with_custom_header() {
        test_hook(
            || {
                let context =
                    TableContext::use_table_context((NameColumn, AgeColumn, PriorityColumn));
                let rows = Signal::new(vec![Person {
                    name: "Alice".to_string(),
                    age: 30,
                }]);
                (context, rows)
            },
            |(context, rows), _| {
                let mut exporter = MockExporter::new();
                context.serialize(rows.into(), &mut exporter).unwrap();

                // PriorityColumn has custom header
                assert_eq!(
                    exporter.headers.as_slice(),
                    &[
                        (0, "Name".to_string()),
                        (1, "Age".to_string()),
                        (2, "Custom Priority Header".to_string()) // PriorityColumn has custom header
                    ]
                );
                assert_eq!(
                    exporter.cells.as_slice(),
                    &[
                        (0, 0, "\"Alice\"".to_string()),
                        (0, 1, "30".to_string()),
                        (0, 2, "\"High\"".to_string()),
                    ]
                );
            },
            |_| {},
        );
    }

    #[test]
    fn test_export_with_column_reordering() {
        test_hook(
            || {
                let context = TableContext::use_table_context((NameColumn, AgeColumn));
                let rows = Signal::new(vec![Person {
                    name: "Alice".to_string(),
                    age: 30,
                }]);
                (context, rows)
            },
            |(context, rows), _| {
                // Swap columns: Age should come before Name
                context.data.swap_columns(0, 1);

                let mut exporter = MockExporter::new();
                context.serialize(rows.into(), &mut exporter).unwrap();

                // Headers should be in reordered position
                assert_eq!(
                    exporter.headers.as_slice(),
                    &[(0, "Age".to_string()), (1, "Name".to_string())]
                );
                // Cells should follow the reordered columns
                assert_eq!(
                    exporter.cells.as_slice(),
                    &[(0, 0, "30".to_string()), (0, 1, "\"Alice\"".to_string())]
                );
            },
            |_| {},
        );
    }

    #[test]
    fn test_export_with_hidden_columns() {
        test_hook(
            || {
                let context = TableContext::use_table_context((NameColumn, AgeColumn));
                let rows = Signal::new(vec![Person {
                    name: "Alice".to_string(),
                    age: 30,
                }]);
                (context, rows)
            },
            |(context, rows), _| {
                // Hide the Age column (index 1)
                context.data.hide_column(1);

                let mut exporter = MockExporter::new();
                context.serialize(rows.into(), &mut exporter).unwrap();

                // Only Name column should be exported
                assert_eq!(exporter.headers.as_slice(), &[(0, "Name".to_string())]);
                assert_eq!(
                    exporter.cells.as_slice(),
                    &[(0, 0, "\"Alice\"".to_string())]
                );
            },
            |_| {},
        );
    }

    #[test]
    fn test_export_with_sorted_rows() {
        test_hook(
            || {
                let context = TableContext::use_table_context((NameColumn, AgeColumn));
                let rows = Signal::new(vec![
                    Person {
                        name: "Charlie".to_string(),
                        age: 35,
                    },
                    Person {
                        name: "Alice".to_string(),
                        age: 30,
                    },
                    Person {
                        name: "Bob".to_string(),
                        age: 25,
                    },
                ]);
                (context, rows)
            },
            |(context, rows), _| {
                // Sort by age (column 1) ascending
                context.data.request_sort(
                    1,
                    SortGesture::AddFirst(Sort {
                        direction: SortDirection::Ascending,
                    }),
                );

                let mut exporter = MockExporter::new();
                context.serialize(rows.into(), &mut exporter).unwrap();

                assert_eq!(
                    exporter.headers.as_slice(),
                    &[(0, "Name".to_string()), (1, "Age".to_string())]
                );
                // Should be sorted: Bob (25), Alice (30), Charlie (35)
                assert_eq!(
                    exporter.cells.as_slice(),
                    &[
                        (0, 0, "\"Bob\"".to_string()),
                        (0, 1, "25".to_string()),
                        (1, 0, "\"Alice\"".to_string()),
                        (1, 1, "30".to_string()),
                        (2, 0, "\"Charlie\"".to_string()),
                        (2, 1, "35".to_string()),
                    ]
                );
            },
            |_| {},
        );
    }

    #[test]
    fn test_export_with_all_columns_hidden() {
        test_hook(
            || {
                let context = TableContext::use_table_context((NameColumn, AgeColumn));
                let rows = Signal::new(vec![Person {
                    name: "Alice".to_string(),
                    age: 30,
                }]);
                (context, rows)
            },
            |(context, rows), _| {
                // Hide all columns
                context.data.hide_column(0);
                context.data.hide_column(1);

                let mut exporter = MockExporter::new();
                context.serialize(rows.into(), &mut exporter).unwrap();

                // No headers or cells should be exported
                assert_eq!(exporter.headers.as_slice(), &[]);
                assert_eq!(exporter.cells.as_slice(), &[]);
            },
            |_| {},
        );
    }

    #[test]
    fn test_export_with_combined_features() {
        test_hook(
            || {
                let context =
                    TableContext::use_table_context((NameColumn, AgeColumn, PriorityColumn));
                let rows = Signal::new(vec![
                    Person {
                        name: "Charlie".to_string(),
                        age: 35,
                    },
                    Person {
                        name: "Alice".to_string(),
                        age: 30,
                    },
                    Person {
                        name: "Bob".to_string(),
                        age: 25,
                    },
                ]);
                (context, rows)
            },
            |(context, rows), _| {
                // 1. Hide Priority column
                context.data.hide_column(2);
                // 2. Swap Name and Age columns
                context.data.swap_columns(0, 1);
                // 3. Sort by age ascending (now at column 0 after swap)
                context.data.request_sort(
                    0,
                    SortGesture::AddFirst(Sort {
                        direction: SortDirection::Ascending,
                    }),
                );

                let mut exporter = MockExporter::new();
                context.serialize(rows.into(), &mut exporter).unwrap();

                // Only Age and Name columns (reordered), sorted by Age
                assert_eq!(
                    exporter.headers.as_slice(),
                    &[(0, "Age".to_string()), (1, "Name".to_string())]
                );
                assert_eq!(
                    exporter.cells.as_slice(),
                    &[
                        (0, 0, "25".to_string()),
                        (0, 1, "\"Bob\"".to_string()),
                        (1, 0, "30".to_string()),
                        (1, 1, "\"Alice\"".to_string()),
                        (2, 0, "35".to_string()),
                        (2, 1, "\"Charlie\"".to_string()),
                    ]
                );
            },
            |_| {},
        );
    }
}
