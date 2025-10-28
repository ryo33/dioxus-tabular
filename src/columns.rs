use crate::{Row, TableColumn, TableContext};
use dioxus::prelude::*;

#[allow(clippy::type_complexity)]
pub trait Columns<R: Row>: Clone + PartialEq + 'static {
    fn column_names(&self) -> Vec<String>;
    fn headers(&self) -> Vec<Box<dyn Fn(&TableContext<Self>, Vec<Attribute>) -> Element + '_>>;
    fn columns(&self) -> Vec<Box<dyn Fn(&TableContext<Self>, &R, Vec<Attribute>) -> Element + '_>>;
    fn filter(&self, row: &R) -> bool;
    fn compare(&self) -> Vec<Box<dyn Fn(&R, &R) -> std::cmp::Ordering + '_>>;
}

impl<A: TableColumn<R>, B: TableColumn<R>, R: Row> Columns<R> for (A, B) {
    fn column_names(&self) -> Vec<String> {
        vec![self.0.column_name(), self.1.column_name()]
    }
    fn headers(&self) -> Vec<Box<dyn Fn(&TableContext<Self>, Vec<Attribute>) -> Element + '_>> {
        vec![
            Box::new(move |context, attributes| {
                self.0
                    .render_header(context.data.column_context(0), attributes)
            }),
            Box::new(move |context, attributes| {
                self.1
                    .render_header(context.data.column_context(1), attributes)
            }),
        ]
    }
    fn columns(&self) -> Vec<Box<dyn Fn(&TableContext<Self>, &R, Vec<Attribute>) -> Element + '_>> {
        vec![
            Box::new(move |context, row, attributes| {
                self.0
                    .render_cell(context.data.column_context(0), row, attributes.clone())
            }),
            Box::new(move |context, row, attributes| {
                self.1
                    .render_cell(context.data.column_context(1), row, attributes)
            }),
        ]
    }
    fn filter(&self, row: &R) -> bool {
        self.0.filter(row) && self.1.filter(row)
    }
    fn compare(&self) -> Vec<Box<dyn Fn(&R, &R) -> std::cmp::Ordering + '_>> {
        vec![
            Box::new(move |a, b| self.0.compare(a, b)),
            Box::new(move |a, b| self.1.compare(a, b)),
        ]
    }
}
