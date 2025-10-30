//! Complete example showing a sortable table with interactive headers.
//!
//! This demonstrates:
//! - Multiple columns with sorting
//! - Interactive sort indicators
//! - Clickable headers to toggle sort
//! - Multiple column types

use dioxus::prelude::*;
use dioxus_tabular::*;

// 1. Define your data model
#[derive(Clone, PartialEq)]
struct User {
    id: u32,
    name: String,
    age: u32,
    email: String,
}

impl Row for User {
    fn key(&self) -> impl Into<String> {
        self.id.to_string()
    }
}

// 2. Define accessor types for type-safe data access
#[derive(Clone, PartialEq)]
struct UserId(u32);

#[derive(Clone, PartialEq)]
struct UserName(String);

#[derive(Clone, PartialEq)]
struct UserAge(u32);

#[derive(Clone, PartialEq)]
struct UserEmail(String);

// 3. Implement GetRowData for each accessor
impl GetRowData<UserId> for User {
    fn get(&self) -> UserId {
        UserId(self.id)
    }
}

impl GetRowData<UserName> for User {
    fn get(&self) -> UserName {
        UserName(self.name.clone())
    }
}

impl GetRowData<UserAge> for User {
    fn get(&self) -> UserAge {
        UserAge(self.age)
    }
}

impl GetRowData<UserEmail> for User {
    fn get(&self) -> UserEmail {
        UserEmail(self.email.clone())
    }
}

// 4. Define columns with sorting capabilities
#[derive(Clone, PartialEq)]
struct IdColumn;

impl<R: Row + GetRowData<UserId>> TableColumn<R> for IdColumn {
    fn column_name(&self) -> String {
        "id".into()
    }

    fn render_header(&self, context: ColumnContext, attributes: Vec<Attribute>) -> Element {
        let sort_indicator = context.sort_info().map(|info| match info.direction {
            SortDirection::Ascending => " ↑",
            SortDirection::Descending => " ↓",
        });

        rsx! {
            th { ..attributes,
                style: "cursor: pointer; user-select: none;",
                onclick: move |_| {
                    context.request_sort(SortGesture::Toggle);
                },
                "ID"
                {sort_indicator}
            }
        }
    }

    fn render_cell(&self, _context: ColumnContext, row: &R, attributes: Vec<Attribute>) -> Element {
        rsx! { td { ..attributes, "{row.get().0}" } }
    }

    fn compare(&self, a: &R, b: &R) -> std::cmp::Ordering {
        a.get().0.cmp(&b.get().0)
    }
}

#[derive(Clone, PartialEq)]
struct NameColumn;

impl<R: Row + GetRowData<UserName>> TableColumn<R> for NameColumn {
    fn column_name(&self) -> String {
        "name".into()
    }

    fn render_header(&self, context: ColumnContext, attributes: Vec<Attribute>) -> Element {
        let sort_indicator = context.sort_info().map(|info| match info.direction {
            SortDirection::Ascending => " ↑",
            SortDirection::Descending => " ↓",
        });

        rsx! {
            th { ..attributes,
                style: "cursor: pointer; user-select: none;",
                onclick: move |_| {
                    // Toggle sort when clicked
                    if context.sort_info().is_some() {
                        context.request_sort(SortGesture::Toggle);
                    } else {
                        context.request_sort(SortGesture::AddFirst(Sort {
                            direction: SortDirection::Ascending,
                        }));
                    }
                },
                "Name"
                {sort_indicator}
            }
        }
    }

    fn render_cell(&self, _context: ColumnContext, row: &R, attributes: Vec<Attribute>) -> Element {
        rsx! { td { ..attributes, "{row.get().0}" } }
    }

    fn compare(&self, a: &R, b: &R) -> std::cmp::Ordering {
        a.get().0.cmp(&b.get().0)
    }
}

#[derive(Clone, PartialEq)]
struct AgeColumn;

impl<R: Row + GetRowData<UserAge>> TableColumn<R> for AgeColumn {
    fn column_name(&self) -> String {
        "age".into()
    }

    fn render_header(&self, context: ColumnContext, attributes: Vec<Attribute>) -> Element {
        let sort_indicator = context.sort_info().map(|info| match info.direction {
            SortDirection::Ascending => " ↑",
            SortDirection::Descending => " ↓",
        });

        rsx! {
            th { ..attributes,
                style: "cursor: pointer; user-select: none;",
                onclick: move |_| {
                    if context.sort_info().is_some() {
                        context.request_sort(SortGesture::Toggle);
                    } else {
                        context.request_sort(SortGesture::AddFirst(Sort {
                            direction: SortDirection::Ascending,
                        }));
                    }
                },
                "Age"
                {sort_indicator}
            }
        }
    }

    fn render_cell(&self, _context: ColumnContext, row: &R, attributes: Vec<Attribute>) -> Element {
        rsx! { td { ..attributes, "{row.get().0}" } }
    }

    fn compare(&self, a: &R, b: &R) -> std::cmp::Ordering {
        a.get().0.cmp(&b.get().0)
    }
}

#[derive(Clone, PartialEq)]
struct EmailColumn;

impl<R: Row + GetRowData<UserEmail>> TableColumn<R> for EmailColumn {
    fn column_name(&self) -> String {
        "email".into()
    }

    fn render_header(&self, context: ColumnContext, attributes: Vec<Attribute>) -> Element {
        let sort_indicator = context.sort_info().map(|info| match info.direction {
            SortDirection::Ascending => " ↑",
            SortDirection::Descending => " ↓",
        });

        rsx! {
            th { ..attributes,
                style: "cursor: pointer; user-select: none;",
                onclick: move |_| {
                    if context.sort_info().is_some() {
                        context.request_sort(SortGesture::Toggle);
                    } else {
                        context.request_sort(SortGesture::AddFirst(Sort {
                            direction: SortDirection::Ascending,
                        }));
                    }
                },
                "Email"
                {sort_indicator}
            }
        }
    }

    fn render_cell(&self, _context: ColumnContext, row: &R, attributes: Vec<Attribute>) -> Element {
        rsx! { td { ..attributes, "{row.get().0}" } }
    }

    fn compare(&self, a: &R, b: &R) -> std::cmp::Ordering {
        a.get().0.cmp(&b.get().0)
    }
}

fn app() -> Element {
    // Sample data
    let users = use_signal(|| {
        vec![
            User {
                id: 1,
                name: "Alice Johnson".to_string(),
                age: 28,
                email: "alice@example.com".to_string(),
            },
            User {
                id: 2,
                name: "Bob Smith".to_string(),
                age: 34,
                email: "bob@example.com".to_string(),
            },
            User {
                id: 3,
                name: "Carol Williams".to_string(),
                age: 25,
                email: "carol@example.com".to_string(),
            },
            User {
                id: 4,
                name: "David Brown".to_string(),
                age: 42,
                email: "david@example.com".to_string(),
            },
            User {
                id: 5,
                name: "Eve Davis".to_string(),
                age: 31,
                email: "eve@example.com".to_string(),
            },
        ]
    });

    // Create table with multiple columns
    let data = use_tabular((IdColumn, NameColumn, AgeColumn, EmailColumn), users.into());

    rsx! {
        style { {include_str!("../examples/styles.css")} }
        div { class: "container",
            h1 { "Sortable User Table" }
            p { "Click on column headers to sort. Click again to toggle between ascending and descending." }

            table { class: "user-table",
                thead {
                    tr { TableHeaders { data } }
                }
                tbody {
                    for row in data.rows() {
                        tr { key: "{row.key()}", TableCells { row } }
                    }
                }
            }
        }
    }
}

fn main() {
    dioxus::launch(app);
}
