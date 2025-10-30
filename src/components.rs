use crate::{Columns, Row, RowData, TableContext, TableData};
use dioxus::prelude::*;

/// Creates a reactive table with the given columns and rows.
///
/// This is the main hook for setting up a table. It returns a [`TableData`] that can be used
/// with [`TableHeaders`] and [`TableCells`] components to render the table.
///
/// # Parameters
///
/// - `columns`: A tuple of columns implementing [`TableColumn`](crate::TableColumn)
/// - `rows`: A reactive signal containing the row data
///
/// # Example
///
/// ```
/// use dioxus::prelude::*;
/// use dioxus_tabular::*;
///
/// # #[derive(Clone, PartialEq)]
/// # struct User { id: u32, name: String }
/// # impl Row for User {
/// #     fn key(&self) -> impl Into<String> { self.id.to_string() }
/// # }
/// # #[derive(Clone, PartialEq)]
/// # struct NameColumn;
/// # #[derive(Clone, PartialEq)]
/// # struct UserName(String);
/// # impl GetRowData<UserName> for User {
/// #     fn get(&self) -> UserName { UserName(self.name.clone()) }
/// # }
/// # impl<R: Row + GetRowData<UserName>> TableColumn<R> for NameColumn {
/// #     fn column_name(&self) -> String { "name".into() }
/// #     fn render_header(&self, _: ColumnContext, _: Vec<Attribute>) -> Element { rsx! { th {} } }
/// #     fn render_cell(&self, _: ColumnContext, row: &R, _: Vec<Attribute>) -> Element {
/// #         rsx! { td { "{row.get().0}" } }
/// #     }
/// # }
/// fn app() -> Element {
///     let users = use_signal(|| vec![
///         User { id: 1, name: "Alice".to_string() },
///         User { id: 2, name: "Bob".to_string() },
///     ]);
///
///     // Create table with single column
///     let data = use_tabular((NameColumn,), users.into());
///
///     rsx! {
///         table {
///             thead { tr { TableHeaders { data } } }
///             tbody {
///                 for row in data.rows() {
///                     tr { key: "{row.key()}", TableCells { row } }
///                 }
///             }
///         }
///     }
/// }
/// ```
///
/// # Multiple Columns
///
/// ```
/// # use dioxus::prelude::*;
/// # use dioxus_tabular::*;
/// # #[derive(Clone, PartialEq)]
/// # struct User { id: u32, name: String }
/// # impl Row for User {
/// #     fn key(&self) -> impl Into<String> { self.id.to_string() }
/// # }
/// # #[derive(Clone, PartialEq)]
/// # struct NameColumn;
/// # #[derive(Clone, PartialEq)]
/// # struct IdColumn;
/// # #[derive(Clone, PartialEq)]
/// # struct UserName(String);
/// # #[derive(Clone, PartialEq)]
/// # struct UserId(u32);
/// # impl GetRowData<UserName> for User {
/// #     fn get(&self) -> UserName { UserName(self.name.clone()) }
/// # }
/// # impl GetRowData<UserId> for User {
/// #     fn get(&self) -> UserId { UserId(self.id) }
/// # }
/// # impl<R: Row + GetRowData<UserName>> TableColumn<R> for NameColumn {
/// #     fn column_name(&self) -> String { "name".into() }
/// #     fn render_header(&self, _: ColumnContext, _: Vec<Attribute>) -> Element { rsx! { th {} } }
/// #     fn render_cell(&self, _: ColumnContext, row: &R, _: Vec<Attribute>) -> Element {
/// #         rsx! { td { "{row.get().0}" } }
/// #     }
/// # }
/// # impl<R: Row + GetRowData<UserId>> TableColumn<R> for IdColumn {
/// #     fn column_name(&self) -> String { "id".into() }
/// #     fn render_header(&self, _: ColumnContext, _: Vec<Attribute>) -> Element { rsx! { th {} } }
/// #     fn render_cell(&self, _: ColumnContext, row: &R, _: Vec<Attribute>) -> Element {
/// #         rsx! { td { "{row.get().0}" } }
/// #     }
/// # }
/// # fn app() -> Element {
/// #     let users = use_signal(|| vec![
/// #         User { id: 1, name: "Alice".to_string() },
/// #     ]);
/// // Use tuple for multiple columns (supports up to 12 columns)
/// let data = use_tabular((IdColumn, NameColumn), users.into());
/// #     rsx! { table {} }
/// # }
/// ```
pub fn use_tabular<C: Columns<R>, R: Row>(
    columns: C,
    rows: ReadOnlySignal<Vec<R>>,
) -> TableData<C, R> {
    let context = TableContext::use_table_context(columns);
    context.table_data(rows)
}

/// Renders table headers for all visible columns.
///
/// This component iterates through the columns and renders each header.
/// It automatically handles column reordering and visibility.
///
/// # Props
///
/// - `data`: The table data from [`use_tabular`]
/// - Additional HTML attributes can be spread onto each `<th>` element
///
/// # Example
///
/// ```
/// # use dioxus::prelude::*;
/// # use dioxus_tabular::*;
/// # #[derive(Clone, PartialEq)]
/// # struct User { id: u32 }
/// # impl Row for User {
/// #     fn key(&self) -> impl Into<String> { self.id.to_string() }
/// # }
/// # #[derive(Clone, PartialEq)]
/// # struct Col;
/// # impl TableColumn<User> for Col {
/// #     fn column_name(&self) -> String { "col".into() }
/// #     fn render_header(&self, _: ColumnContext, _: Vec<Attribute>) -> Element { rsx! { th {} } }
/// #     fn render_cell(&self, _: ColumnContext, _: &User, _: Vec<Attribute>) -> Element { rsx! { td {} } }
/// # }
/// # fn app() -> Element {
/// #     let users = use_signal(|| vec![User { id: 1 }]);
/// #     let data = use_tabular((Col,), users.into());
/// rsx! {
///     table {
///         thead {
///             tr {
///                 // Renders all column headers
///                 TableHeaders { data }
///             }
///         }
///     }
/// }
/// # }
/// ```
///
/// # With Custom Attributes
///
/// ```
/// # use dioxus::prelude::*;
/// # use dioxus_tabular::*;
/// # #[derive(Clone, PartialEq)]
/// # struct User { id: u32 }
/// # impl Row for User {
/// #     fn key(&self) -> impl Into<String> { self.id.to_string() }
/// # }
/// # #[derive(Clone, PartialEq)]
/// # struct Col;
/// # impl TableColumn<User> for Col {
/// #     fn column_name(&self) -> String { "col".into() }
/// #     fn render_header(&self, _: ColumnContext, _: Vec<Attribute>) -> Element { rsx! { th {} } }
/// #     fn render_cell(&self, _: ColumnContext, _: &User, _: Vec<Attribute>) -> Element { rsx! { td {} } }
/// # }
/// # fn app() -> Element {
/// #     let users = use_signal(|| vec![User { id: 1 }]);
/// #     let data = use_tabular((Col,), users.into());
/// rsx! {
///     tr {
///         TableHeaders {
///             data,
///             class: "header-cell",
///             style: "font-weight: bold;"
///         }
///     }
/// }
/// # }
/// ```
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

/// Renders table cells for a single row across all visible columns.
///
/// This component iterates through the columns and renders each cell for the given row.
/// It automatically handles column reordering and visibility.
///
/// # Props
///
/// - `row`: A row from iterating over `data.rows()`
/// - Additional HTML attributes can be spread onto each `<td>` element
///
/// # Example
///
/// ```
/// # use dioxus::prelude::*;
/// # use dioxus_tabular::*;
/// # #[derive(Clone, PartialEq)]
/// # struct User { id: u32 }
/// # impl Row for User {
/// #     fn key(&self) -> impl Into<String> { self.id.to_string() }
/// # }
/// # #[derive(Clone, PartialEq)]
/// # struct Col;
/// # impl TableColumn<User> for Col {
/// #     fn column_name(&self) -> String { "col".into() }
/// #     fn render_header(&self, _: ColumnContext, _: Vec<Attribute>) -> Element { rsx! { th {} } }
/// #     fn render_cell(&self, _: ColumnContext, _: &User, _: Vec<Attribute>) -> Element { rsx! { td {} } }
/// # }
/// # fn app() -> Element {
/// #     let users = use_signal(|| vec![User { id: 1 }]);
/// #     let data = use_tabular((Col,), users.into());
/// rsx! {
///     table {
///         tbody {
///             for row in data.rows() {
///                 tr {
///                     key: "{row.key()}",
///                     // Renders all cells for this row
///                     TableCells { row }
///                 }
///             }
///         }
///     }
/// }
/// # }
/// ```
///
/// # With Custom Attributes
///
/// ```
/// # use dioxus::prelude::*;
/// # use dioxus_tabular::*;
/// # #[derive(Clone, PartialEq)]
/// # struct User { id: u32 }
/// # impl Row for User {
/// #     fn key(&self) -> impl Into<String> { self.id.to_string() }
/// # }
/// # #[derive(Clone, PartialEq)]
/// # struct Col;
/// # impl TableColumn<User> for Col {
/// #     fn column_name(&self) -> String { "col".into() }
/// #     fn render_header(&self, _: ColumnContext, _: Vec<Attribute>) -> Element { rsx! { th {} } }
/// #     fn render_cell(&self, _: ColumnContext, _: &User, _: Vec<Attribute>) -> Element { rsx! { td {} } }
/// # }
/// # fn app() -> Element {
/// #     let users = use_signal(|| vec![User { id: 1 }]);
/// #     let data = use_tabular((Col,), users.into());
/// rsx! {
///     for row in data.rows() {
///         tr {
///             key: "{row.key()}",
///             TableCells {
///                 row,
///                 class: "data-cell",
///                 style: "padding: 8px;"
///             }
///         }
///     }
/// }
/// # }
/// ```
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
