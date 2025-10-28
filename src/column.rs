use crate::{ColumnContext, Row};
use dioxus::prelude::*;

pub trait TableColumn<R: Row>: Clone + PartialEq + 'static {
    /// The name of the column name in the header of export feature.
    fn column_name(&self) -> String;
    /// Serialize the column value for export feature.
    fn serialize(&self, row: &R) -> String;
    fn render_header(&self, context: ColumnContext, attributes: Vec<Attribute>) -> Element;
    fn render_cell(&self, context: ColumnContext, row: &R, attributes: Vec<Attribute>) -> Element;
    fn filter(&self, row: &R) -> bool {
        let _ = row;
        true
    }
    fn compare(&self, a: &R, b: &R) -> std::cmp::Ordering {
        let _ = (a, b);
        std::cmp::Ordering::Equal
    }
}
