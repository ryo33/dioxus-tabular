use crate::{ColumnContext, Row};
use dioxus::prelude::*;

/// Describes how a single column renders, filters, and sorts rows.
///
/// This is the main trait you implement to define a column's behavior. Each column
/// is a self-contained unit that can:
/// - Render its header and cells
/// - Filter rows based on custom logic
/// - Sort rows with custom comparison
/// - Hold its own reactive state via `Signal`
///
/// # Type Parameter
///
/// - `R`: The row type this column works with (must implement [`Row`])
///
/// # Example
///
/// ```
/// use dioxus::prelude::*;
/// use dioxus_tabular::*;
///
/// #[derive(Clone, PartialEq)]
/// struct Product {
///     id: u32,
///     name: String,
///     price: u32,
/// }
///
/// impl Row for Product {
///     fn key(&self) -> impl Into<String> {
///         self.id.to_string()
///     }
/// }
///
/// #[derive(Clone, PartialEq)]
/// struct Price(u32);
///
/// impl GetRowData<Price> for Product {
///     fn get(&self) -> Price {
///         Price(self.price)
///     }
/// }
///
/// // A column that displays and sorts by price
/// #[derive(Clone, PartialEq)]
/// struct PriceColumn;
///
/// impl<R: Row + GetRowData<Price>> TableColumn<R> for PriceColumn {
///     fn column_name(&self) -> String {
///         "price".into()
///     }
///
///     fn render_header(&self, _context: ColumnContext, attributes: Vec<Attribute>) -> Element {
///         rsx! { th { ..attributes, "Price" } }
///     }
///
///     fn render_cell(&self, _context: ColumnContext, row: &R, attributes: Vec<Attribute>) -> Element {
///         rsx! { td { ..attributes, "¥{row.get().0}" } }
///     }
///
///     fn compare(&self, a: &R, b: &R) -> std::cmp::Ordering {
///         a.get().0.cmp(&b.get().0)
///     }
/// }
/// ```
///
/// # Filtering Example
///
/// ```
/// # use dioxus::prelude::*;
/// # use dioxus_tabular::*;
/// # #[derive(Clone, PartialEq)]
/// # struct Product { price: u32, id: u32 }
/// # impl Row for Product {
/// #     fn key(&self) -> impl Into<String> { self.id.to_string() }
/// # }
/// # #[derive(Clone, PartialEq)]
/// # struct Price(u32);
/// # impl GetRowData<Price> for Product {
/// #     fn get(&self) -> Price { Price(self.price) }
/// # }
/// #[derive(Clone, PartialEq)]
/// struct PriceColumn {
///     min_price: Signal<u32>,
/// }
///
/// impl<R: Row + GetRowData<Price>> TableColumn<R> for PriceColumn {
///     fn column_name(&self) -> String {
///         "price".into()
///     }
///
///     fn render_header(&self, _context: ColumnContext, attributes: Vec<Attribute>) -> Element {
///         rsx! { th { ..attributes, "Price" } }
///     }
///
///     fn render_cell(&self, _context: ColumnContext, row: &R, attributes: Vec<Attribute>) -> Element {
///         rsx! { td { ..attributes, "¥{row.get().0}" } }
///     }
///
///     // Only show rows with price >= min_price
///     fn filter(&self, row: &R) -> bool {
///         row.get().0 >= *self.min_price.read()
///     }
/// }
/// ```
pub trait TableColumn<R: Row>: Clone + PartialEq + 'static {
    /// Returns the unique name of this column.
    ///
    /// This name is used as a key for the column and should be unique within a table.
    fn column_name(&self) -> String;

    /// Renders the column header.
    ///
    /// Use `context.sort_info()` to display sort state, and spread `attributes` onto your header element.
    /// Typically returns a `<th>` element.
    fn render_header(&self, context: ColumnContext, attributes: Vec<Attribute>) -> Element;

    /// Renders a cell for the given row.
    ///
    /// Access row data via `GetRowData::get()`, and spread `attributes` onto your cell element.
    /// Typically returns a `<td>` element.
    fn render_cell(&self, context: ColumnContext, row: &R, attributes: Vec<Attribute>) -> Element;

    /// Determines whether a row should be displayed.
    ///
    /// Return `true` to include the row, `false` to filter it out.
    /// All column filters are combined with AND logic.
    /// Default: includes all rows.
    fn filter(&self, row: &R) -> bool {
        let _ = row;
        true
    }

    /// Compares two rows for sorting.
    ///
    /// Return `Ordering::Less` if `a < b`, `Ordering::Greater` if `a > b`, or `Ordering::Equal`.
    /// Used for multi-column sorting when this column is in the sort list.
    /// Default: considers all rows equal (no sorting).
    fn compare(&self, a: &R, b: &R) -> std::cmp::Ordering {
        let _ = (a, b);
        std::cmp::Ordering::Equal
    }
}
