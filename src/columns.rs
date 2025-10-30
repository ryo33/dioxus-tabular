#[cfg(feature = "export")]
use crate::Exporter;
use crate::{Row, TableColumn, TableContext};
use dioxus::prelude::*;

/// Trait automatically implemented for tuples of [`TableColumn`]s.
///
/// This trait is implemented for tuples of 1 to 12 columns via macros.
/// You don't need to implement this trait manually.
///
/// # Example
///
/// ```
/// # use dioxus_tabular::*;
/// # #[derive(Clone, PartialEq)]
/// # struct Col1;
/// # #[derive(Clone, PartialEq)]
/// # struct Col2;
/// # #[derive(Clone, PartialEq)]
/// # struct User { id: u32 }
/// # impl Row for User {
/// #     fn key(&self) -> impl Into<String> { self.id.to_string() }
/// # }
/// # impl TableColumn<User> for Col1 {
/// #     fn column_name(&self) -> String { "col1".into() }
/// #     fn render_header(&self, _: ColumnContext, _: Vec<dioxus::prelude::Attribute>) -> dioxus::prelude::Element { todo!() }
/// #     fn render_cell(&self, _: ColumnContext, _: &User, _: Vec<dioxus::prelude::Attribute>) -> dioxus::prelude::Element { todo!() }
/// # }
/// # impl TableColumn<User> for Col2 {
/// #     fn column_name(&self) -> String { "col2".into() }
/// #     fn render_header(&self, _: ColumnContext, _: Vec<dioxus::prelude::Attribute>) -> dioxus::prelude::Element { todo!() }
/// #     fn render_cell(&self, _: ColumnContext, _: &User, _: Vec<dioxus::prelude::Attribute>) -> dioxus::prelude::Element { todo!() }
/// # }
/// // Single column
/// let cols1 = Col1;
///
/// // Multiple columns (tuple)
/// let cols2 = (Col1, Col2);
/// ```
#[allow(clippy::type_complexity, reason = "to provide internal API")]
pub trait Columns<R: Row>: Clone + PartialEq + 'static {
    /// Returns the names of all columns.
    fn column_names(&self) -> Vec<String>;
    /// Returns header renderers for all columns.
    fn headers(&self) -> Vec<Box<dyn Fn(&TableContext<Self>, Vec<Attribute>) -> Element + '_>>;
    /// Returns cell renderers for all columns.
    fn columns(&self) -> Vec<Box<dyn Fn(&TableContext<Self>, &R, Vec<Attribute>) -> Element + '_>>;
    /// Returns true if the row passes all column filters.
    fn filter(&self, row: &R) -> bool;
    /// Returns comparators for all columns.
    fn compare(&self) -> Vec<Box<dyn Fn(&R, &R) -> std::cmp::Ordering + '_>>;
}

/// Trait for columns that support serialization (export feature).
///
/// Automatically implemented for tuples of [`SerializableColumn`](crate::SerializableColumn)s.
#[cfg(feature = "export")]
pub trait SerializableColumns<R: Row>: Columns<R> {
    /// Returns header serializers for all columns.
    fn serialize_headers(&self) -> Vec<Box<dyn Fn() -> String + '_>>;
    /// Returns cell serializers for all columns.
    #[allow(clippy::type_complexity, reason = "to provide internal API")]
    fn serialize_cell<E: Exporter>(
        &self,
    ) -> Vec<Box<dyn Fn(usize, usize, &R, &mut E) -> Result<(), E::Error> + '_>>;
}

macro_rules! columns {
    ($($number:tt => $column:ident),*) => {
        impl<$($column: TableColumn<R>),*, R: Row> Columns<R> for ($($column),*,) {
            fn column_names(&self) -> Vec<String> {
                vec![$(self.$number.column_name()),*]
            }
            fn headers(&self) -> Vec<Box<dyn Fn(&TableContext<Self>, Vec<Attribute>) -> Element + '_>> {
                vec![$(Box::new(move |context, attributes| {
                    self.$number.render_header(context.data.column_context($number), attributes)
                })),*]
            }
            fn columns(&self) -> Vec<Box<dyn Fn(&TableContext<Self>, &R, Vec<Attribute>) -> Element + '_>> {
                vec![$(Box::new(move |context, row, attributes| {
                    self.$number.render_cell(context.data.column_context($number), row, attributes)
                })),*]
            }
            fn filter(&self, row: &R) -> bool {
                $(self.$number.filter(row) &&)* true
            }
            fn compare(&self) -> Vec<Box<dyn Fn(&R, &R) -> std::cmp::Ordering + '_>> {
                vec![$(Box::new(move |a, b| self.$number.compare(a, b))),*]
            }
        }
        #[cfg(feature = "export")]
        serialize_columns!($($number => $column),*);
    }
}

macro_rules! serialize_columns {
    ($($number:tt => $column:ident),*) => {
        impl<$($column: crate::SerializableColumn<R>),*, R: Row> SerializableColumns<R> for ($($column),*,) {
            fn serialize_headers(&self) -> Vec<Box<dyn Fn() -> String + '_>> {
                vec![$(Box::new(move || self.$number.header())),*]
            }
            fn serialize_cell<Ex: Exporter>(&self) -> Vec<Box<dyn Fn(usize, usize, &R, &mut Ex) -> Result<(), Ex::Error> + '_>> {
                vec![$(Box::new(move |row_index, col_index, row, exporter| exporter.serialize_cell(row_index, col_index, self.$number.serialize_cell(row)))),*]
            }
        }
    }
}

columns!(0 => A);
columns!(0 => A, 1 => B);
columns!(0 => A, 1 => B, 2 => C);
columns!(0 => A, 1 => B, 2 => C, 3 => D);
columns!(0 => A, 1 => B, 2 => C, 3 => D, 4 => E);
columns!(0 => A, 1 => B, 2 => C, 3 => D, 4 => E, 5 => F);
columns!(0 => A, 1 => B, 2 => C, 3 => D, 4 => E, 5 => F, 6 => G);
columns!(0 => A, 1 => B, 2 => C, 3 => D, 4 => E, 5 => F, 6 => G, 7 => H);
columns!(0 => A, 1 => B, 2 => C, 3 => D, 4 => E, 5 => F, 6 => G, 7 => H, 8 => I);
columns!(0 => A, 1 => B, 2 => C, 3 => D, 4 => E, 5 => F, 6 => G, 7 => H, 8 => I, 9 => J);
columns!(0 => A, 1 => B, 2 => C, 3 => D, 4 => E, 5 => F, 6 => G, 7 => H, 8 => I, 9 => J, 10 => K);
columns!(0 => A, 1 => B, 2 => C, 3 => D, 4 => E, 5 => F, 6 => G, 7 => H, 8 => I, 9 => J, 10 => K, 11 => L);
