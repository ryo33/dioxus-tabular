use crate::{Row, TableColumn, TableContext};
use dioxus::prelude::*;

#[allow(clippy::type_complexity)]
pub trait Columns<R: Row>: Clone + PartialEq + 'static {
    fn column_names(&self) -> Vec<String>;
    fn headers(&self) -> Vec<Box<dyn Fn(TableContext) -> Element + '_>>;
    fn columns(&self) -> Vec<Box<dyn Fn(TableContext, &R, Vec<Attribute>) -> Element + '_>>;
    fn filter(&self, row: &R) -> bool;
    fn compare(&self) -> Vec<Box<dyn Fn(&R, &R) -> std::cmp::Ordering + '_>>;

    /// This only renders the inner content of the header row, not the outer <thead> and <tr> tags.
    fn render_headers(&self, context: TableContext) -> Element {
        rsx! {
            for (name , render) in self.column_names().into_iter().zip(self.headers()) {
                Fragment { key: "{name}", {render(context)} }
            }
        }
    }

    /// This only renders the inner content of the row, not the outer <tbody> and <tr> tags.
    fn render_columns(
        &self,
        context: TableContext,
        row: &R,
        attributes: Vec<Attribute>,
    ) -> Element {
        rsx! {
            for (name , render) in self.column_names().into_iter().zip(self.columns()) {
                Fragment { key: "{name}", {render(context, row, attributes.clone())} }
            }
        }
    }
}

impl<A: TableColumn<R>, B: TableColumn<R>, R: Row> Columns<R> for (A, B) {
    fn column_names(&self) -> Vec<String> {
        vec![self.0.column_name(), self.1.column_name()]
    }
    fn headers(&self) -> Vec<Box<dyn Fn(TableContext) -> Element + '_>> {
        vec![
            Box::new(move |context| self.0.render_header(context.column_context(0))),
            Box::new(move |context| self.1.render_header(context.column_context(1))),
        ]
    }
    fn columns(&self) -> Vec<Box<dyn Fn(TableContext, &R, Vec<Attribute>) -> Element + '_>> {
        vec![
            Box::new(move |context, row, attributes| {
                self.0
                    .render_cell(context.column_context(0), row, attributes.clone())
            }),
            Box::new(move |context, row, attributes| {
                self.1
                    .render_cell(context.column_context(1), row, attributes)
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
