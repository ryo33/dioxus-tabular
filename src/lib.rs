//! Type-safe and composable table framework for Dioxus.
//!
//! `dioxus-tabular` provides a declarative, type-safe way to build reactive tables in Dioxus.
//! Instead of configuring tables with dynamic descriptors, you define columns as typed components
//! that own their rendering, filtering, and sorting logic.
//!
//! # Core Concepts
//!
//! - **[`Row`]**: Trait defining the unique key and identity of each row
//! - **[`GetRowData<T>`]**: Trait providing typed access to row data
//! - **[`TableColumn`]**: Trait describing how a column renders, filters, and sorts
//! - **[`Columns`]**: Automatically implemented for tuples of `TableColumn`s
//! - **[`use_tabular`]**: Hook to create a reactive table
//! - **[`TableHeaders`]** / **[`TableCells`]**: Components for rendering headers and cells
//!
//! # Quick Start
//!
//! ```rust
//! use dioxus::prelude::*;
//! use dioxus_tabular::*;
//!
//! // 1. Define your row type
//! #[derive(Clone, PartialEq)]
//! struct User {
//!     id: u32,
//!     name: String,
//! }
//!
//! // 2. Implement Row trait
//! impl Row for User {
//!     fn key(&self) -> impl Into<String> {
//!         self.id.to_string()
//!     }
//! }
//!
//! // 3. Define data accessor types
//! #[derive(Clone, PartialEq)]
//! struct UserName(String);
//!
//! impl GetRowData<UserName> for User {
//!     fn get(&self) -> UserName {
//!         UserName(self.name.clone())
//!     }
//! }
//!
//! // 4. Define a column
//! #[derive(Clone, PartialEq)]
//! struct NameColumn;
//!
//! impl<R: Row + GetRowData<UserName>> TableColumn<R> for NameColumn {
//!     fn column_name(&self) -> String {
//!         "name".into()
//!     }
//!
//!     fn render_header(&self, _context: ColumnContext, attributes: Vec<Attribute>) -> Element {
//!         rsx! { th { ..attributes, "Name" } }
//!     }
//!
//!     fn render_cell(&self, _context: ColumnContext, row: &R, attributes: Vec<Attribute>) -> Element {
//!         rsx! { td { ..attributes, "{row.get().0}" } }
//!     }
//! }
//!
//! // 5. Use in your component
//! fn app() -> Element {
//!     let users = use_signal(|| vec![
//!         User { id: 1, name: "Alice".to_string() },
//!         User { id: 2, name: "Bob".to_string() },
//!     ]);
//!
//!     let data = use_tabular((NameColumn,), users.into());
//!
//!     rsx! {
//!         table {
//!             thead { tr { TableHeaders { data } } }
//!             tbody {
//!                 for row in data.rows() {
//!                     tr { key: "{row.key()}", TableCells { row } }
//!                 }
//!             }
//!         }
//!     }
//! }
//! ```
//!
//! # Features
//!
//! ## Multi-Column Sorting
//!
//! Columns can implement custom comparison logic via [`TableColumn::compare`].
//! Users can sort by multiple columns with priority control using [`ColumnContext::request_sort`].
//!
//! ## Row Filtering
//!
//! Columns can implement filtering logic via [`TableColumn::filter`].
//! Filters are automatically applied when rendering rows.
//!
//! ## Column Ordering and Visibility
//!
//! Control which columns are displayed and in what order using methods on [`ColumnContext`]:
//! - `hide()` / `show()` - Toggle column visibility
//! - `move_to()`, `move_forward()`, `move_backward()` - Reorder columns
//! - `reset_order()` - Restore default state
//!
//! ## Export (optional feature)
//!
//! Enable the `export` feature to serialize table data:
//!
//! ```toml
//! dioxus-tabular = { version = "0.1", features = ["export"] }
//! ```
//!
//! Implement [`SerializableColumn`] and use the [`Exporter`] trait to export to various formats.

mod column;
mod columns;
mod components;
mod context;
#[cfg(feature = "export")]
mod export;
mod row;

#[cfg(test)]
pub mod test_suite;

pub use column::*;
pub use columns::*;
pub use components::*;
pub use context::*;
#[cfg(feature = "export")]
pub use export::*;
pub use row::*;
