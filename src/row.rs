/// Defines the unique key and identity of each table row.
///
/// This trait must be implemented for any type you want to use as a row in a table.
/// The key is used by Dioxus for efficient rendering and should uniquely identify each row.
///
/// # Example
///
/// ```
/// use dioxus_tabular::Row;
///
/// #[derive(Clone, PartialEq)]
/// struct User {
///     id: u32,
///     name: String,
/// }
///
/// impl Row for User {
///     fn key(&self) -> impl Into<String> {
///         self.id.to_string()
///     }
/// }
/// ```
pub trait Row: PartialEq + Clone + 'static {
    /// Returns a unique key for this row.
    ///
    /// The key must be unique within the table and stable across re-renders.
    fn key(&self) -> impl Into<String>;
}

/// Provides typed access to row data.
///
/// This trait allows columns to extract specific data from rows in a type-safe way.
/// Implement this trait for each piece of data you want to access in your columns.
///
/// # Type Parameter
///
/// - `T`: The type of data to extract from the row
///
/// # Example
///
/// ```
/// use dioxus_tabular::GetRowData;
///
/// #[derive(Clone, PartialEq)]
/// struct User {
///     id: u32,
///     name: String,
///     email: String,
/// }
///
/// // Define accessor types
/// #[derive(Clone, PartialEq)]
/// struct UserName(String);
///
/// #[derive(Clone, PartialEq)]
/// struct UserEmail(String);
///
/// // Implement GetRowData for each accessor
/// impl GetRowData<UserName> for User {
///     fn get(&self) -> UserName {
///         UserName(self.name.clone())
///     }
/// }
///
/// impl GetRowData<UserEmail> for User {
///     fn get(&self) -> UserEmail {
///         UserEmail(self.email.clone())
///     }
/// }
/// ```
pub trait GetRowData<T> {
    /// Extracts data of type `T` from this row.
    fn get(&self) -> T;
}
