//! Common types and utilities for documentation examples.
//!
//! This module provides pre-defined types to reduce boilerplate in documentation examples.

use crate::{GetRowData, Row, TableColumn};
use dioxus::prelude::*;

/// Example user type for documentation.
#[derive(Clone, PartialEq)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub age: u32,
}

impl Row for User {
    fn key(&self) -> impl Into<String> {
        self.id.to_string()
    }
}

/// Accessor for user ID.
#[derive(Clone, PartialEq)]
pub struct UserId(pub u32);

impl GetRowData<UserId> for User {
    fn get(&self) -> UserId {
        UserId(self.id)
    }
}

/// Accessor for user name.
#[derive(Clone, PartialEq)]
pub struct UserName(pub String);

impl GetRowData<UserName> for User {
    fn get(&self) -> UserName {
        UserName(self.name.clone())
    }
}

/// Accessor for user age.
#[derive(Clone, PartialEq)]
pub struct UserAge(pub u32);

impl GetRowData<UserAge> for User {
    fn get(&self) -> UserAge {
        UserAge(self.age)
    }
}

/// Example product type for documentation.
#[derive(Clone, PartialEq)]
pub struct Product {
    pub id: u32,
    pub name: String,
    pub price: u32,
}

impl Row for Product {
    fn key(&self) -> impl Into<String> {
        self.id.to_string()
    }
}

/// Accessor for product price.
#[derive(Clone, PartialEq)]
pub struct Price(pub u32);

impl GetRowData<Price> for Product {
    fn get(&self) -> Price {
        Price(self.price)
    }
}

/// Accessor for product name.
#[derive(Clone, PartialEq)]
pub struct ProductName(pub String);

impl GetRowData<ProductName> for Product {
    fn get(&self) -> ProductName {
        ProductName(self.name.clone())
    }
}

/// Simple example column for documentation.
#[derive(Clone, PartialEq)]
pub struct ExampleColumn;

impl<R: Row + GetRowData<UserName>> TableColumn<R> for ExampleColumn {
    fn column_name(&self) -> String {
        "example".into()
    }

    fn render_header(
        &self,
        _context: crate::ColumnContext,
        attributes: Vec<Attribute>,
    ) -> Element {
        rsx! { th { ..attributes, "Example" } }
    }

    fn render_cell(
        &self,
        _context: crate::ColumnContext,
        row: &R,
        attributes: Vec<Attribute>,
    ) -> Element {
        rsx! { td { ..attributes, "{row.get().0}" } }
    }
}
