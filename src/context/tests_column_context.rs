//! Tests for ColumnContext usage patterns (simulating real app usage)

use crate::test_suite::test_hook;
use crate::{ColumnContext, Row, Sort, SortDirection, SortGesture, TableColumn, TableContext};
use dioxus::prelude::*;

// Test helper types for ColumnContext testing
#[derive(Clone, Copy, PartialEq)]
enum ColumnAction {
    Sort(SortGesture),
    SwapWith(usize),
    Hide,
    Show(Option<usize>),
    MoveTo(usize),
}

#[derive(Clone, Copy, PartialEq)]
enum ColumnKind {
    Name,
    Age,
    Priority,
}

#[derive(Clone, PartialEq)]
struct TestColumn {
    kind: ColumnKind,
    trigger: Signal<Option<ColumnAction>>,
}

impl TestColumn {
    fn name(trigger: Signal<Option<ColumnAction>>) -> Self {
        Self {
            kind: ColumnKind::Name,
            trigger,
        }
    }

    fn age(trigger: Signal<Option<ColumnAction>>) -> Self {
        Self {
            kind: ColumnKind::Age,
            trigger,
        }
    }

    fn priority(trigger: Signal<Option<ColumnAction>>) -> Self {
        Self {
            kind: ColumnKind::Priority,
            trigger,
        }
    }
}

impl TableColumn<TestPerson> for TestColumn {
    fn column_name(&self) -> String {
        match self.kind {
            ColumnKind::Name => "Name".to_string(),
            ColumnKind::Age => "Age".to_string(),
            ColumnKind::Priority => "Priority".to_string(),
        }
    }

    fn render_header(&self, context: ColumnContext, attributes: Vec<Attribute>) -> Element {
        // Simulate real app pattern: use_effect triggered by signal changes
        let trigger = self.trigger;
        let kind = self.kind;
        use_effect(move || {
            if let Some(action) = trigger.read().as_ref() {
                match action {
                    ColumnAction::Sort(gesture) => context.request_sort(*gesture),
                    ColumnAction::SwapWith(other) => context.swap_with(*other),
                    ColumnAction::Hide => context.hide(),
                    ColumnAction::Show(at) => context.show(*at),
                    ColumnAction::MoveTo(pos) => context.move_to(*pos),
                }
            }
        });
        let name = match kind {
            ColumnKind::Name => "Name",
            ColumnKind::Age => "Age",
            ColumnKind::Priority => "Priority",
        };
        rsx! {
            th { ..attributes,"{name}" }
        }
    }

    fn render_cell(
        &self,
        _context: ColumnContext,
        row: &TestPerson,
        _attributes: Vec<Attribute>,
    ) -> Element {
        rsx! {
            td { "{row.key().into()}" }
        }
    }

    fn compare(&self, a: &TestPerson, b: &TestPerson) -> std::cmp::Ordering {
        match self.kind {
            ColumnKind::Name => a.name.cmp(&b.name),
            ColumnKind::Age => a.age.cmp(&b.age),
            ColumnKind::Priority => std::cmp::Ordering::Equal,
        }
    }
}

// Test data
#[derive(Debug, Clone, PartialEq)]
struct TestPerson {
    name: String,
    age: u32,
}

impl Row for TestPerson {
    fn key(&self) -> impl Into<String> {
        format!("{}_{}", self.name, self.age)
    }
}

#[test]
fn test_column_context_swap_and_sort() {
    // Test that column reordering (via ColumnContext) doesn't affect sorting
    test_hook(
        || {
            let trigger_name = use_signal(|| None);
            let trigger_age = use_signal(|| None);
            let col_name = TestColumn::name(trigger_name);
            let col_age = TestColumn::age(trigger_age);

            let context = TableContext::use_table_context((col_name, col_age));
            let rows = use_signal(|| {
                vec![
                    TestPerson {
                        name: "Charlie".to_string(),
                        age: 35,
                    },
                    TestPerson {
                        name: "Bob".to_string(),
                        age: 25,
                    },
                    TestPerson {
                        name: "Alice".to_string(),
                        age: 30,
                    },
                ]
            });

            (context, rows, trigger_name, trigger_age)
        },
        |(context, rows, mut trigger_name, _trigger_age), proxy| match proxy.generation {
            0 => {
                // Render headers to register use_effect
                for header in context.headers() {
                    let _ = header.render(vec![]);
                }

                // Trigger actions via ColumnContext (simulates onclick)
                trigger_name.set(Some(ColumnAction::SwapWith(1))); // Swap Name with Age
                trigger_name.set(Some(ColumnAction::Sort(SortGesture::AddFirst(Sort {
                    direction: SortDirection::Ascending,
                })))); // Sort by Name (original column 0)
            }
            1 => {
                // Verify results after use_effect completes
                let sorted_names: Vec<String> = context
                    .rows(rows.into())
                    .map(|row_data| row_data.rows.read()[row_data.index].name.clone())
                    .collect();

                assert_eq!(
                    sorted_names,
                    vec![
                        "Alice".to_string(),
                        "Bob".to_string(),
                        "Charlie".to_string()
                    ],
                    "After swap via ColumnContext, sorting by Name column still works correctly"
                );
            }
            _ => panic!("Unexpected generation: {}", proxy.generation),
        },
        |proxy| {
            assert_eq!(proxy.generation, 1, "Expected exactly 1 generation");
        },
    );
}

#[test]
fn test_column_context_hide() {
    // Test hiding a column via ColumnContext
    test_hook(
        || {
            let trigger_name = use_signal(|| None);
            let trigger_age = use_signal(|| None);
            let col_name = TestColumn::name(trigger_name);
            let col_age = TestColumn::age(trigger_age);

            let context = TableContext::use_table_context((col_name, col_age));

            (context, trigger_name, trigger_age)
        },
        |(context, mut trigger_name, _trigger_age), proxy| match proxy.generation {
            0 => {
                // Render headers to register use_effect
                for header in context.headers() {
                    let _ = header.render(vec![]);
                }

                // Initially, both columns should be visible
                let initial_headers: Vec<_> = context.headers().collect();
                assert_eq!(initial_headers.len(), 2, "Initially should have 2 columns");

                // Hide Name column via ColumnContext
                trigger_name.set(Some(ColumnAction::Hide));
            }
            1 => {
                // Verify Name column is hidden
                let visible_headers: Vec<_> = context.headers().map(|h| h.key()).collect();
                assert_eq!(
                    visible_headers,
                    vec!["Age"],
                    "After hiding Name, should have 1 column"
                );
            }
            _ => panic!("Unexpected generation: {}", proxy.generation),
        },
        |proxy| {
            assert_eq!(proxy.generation, 1, "Expected exactly 1 generation");
        },
    );
}

#[test]
fn test_column_context_show() {
    // Test showing a hidden column via ColumnContext
    test_hook(
        || {
            let trigger_name = use_signal(|| None);
            let trigger_age = use_signal(|| None);
            let col_name = TestColumn::name(trigger_name);
            let col_age = TestColumn::age(trigger_age);

            let context = TableContext::use_table_context((col_name, col_age));

            // Hide Name column initially
            context.data.hide_column(0);

            (context, trigger_name, trigger_age)
        },
        |(context, mut trigger_name, _trigger_age), proxy| match proxy.generation {
            0 => {
                // Render headers to register use_effect
                for header in context.headers() {
                    let _ = header.render(vec![]);
                }

                // Initially, Name should be hidden
                let initial_headers: Vec<_> = context.headers().collect();
                assert_eq!(
                    initial_headers.len(),
                    1,
                    "Initially should have 1 column (Age)"
                );

                // Show Name column via ColumnContext
                trigger_name.set(Some(ColumnAction::Show(None)));
            }
            1 => {
                // Verify Name column is now visible
                let visible_headers: Vec<_> = context.headers().map(|h| h.key()).collect();
                assert_eq!(
                    visible_headers,
                    vec!["Name", "Age"],
                    "After showing Name, should have 2 columns"
                );
            }
            _ => panic!("Unexpected generation: {}", proxy.generation),
        },
        |proxy| {
            assert_eq!(proxy.generation, 1, "Expected exactly 1 generation");
        },
    );
}

#[test]
fn test_column_context_move_to() {
    // Test moving a column via ColumnContext
    test_hook(
        || {
            let trigger_name = use_signal(|| None);
            let trigger_age = use_signal(|| None);
            let trigger_priority = use_signal(|| None);
            let col_name = TestColumn::name(trigger_name);
            let col_age = TestColumn::age(trigger_age);
            let col_priority = TestColumn::priority(trigger_priority);

            let context = TableContext::use_table_context((col_name, col_age, col_priority));

            (context, trigger_priority)
        },
        |(context, mut trigger_priority), proxy| match proxy.generation {
            0 => {
                // Render headers to register use_effect
                for header in context.headers() {
                    let _ = header.render(vec![]);
                }

                // Initial order: Name, Age, Priority (indices 0, 1, 2)
                let initial_headers: Vec<_> = context.headers().collect();
                assert_eq!(initial_headers.len(), 3, "Should have 3 columns");

                // Move Priority (column 2) to position 0 via ColumnContext
                trigger_priority.set(Some(ColumnAction::MoveTo(0)));
            }
            1 => {
                // Verify Priority is now at position 0
                let headers: Vec<_> = context.headers().map(|h| h.key()).collect();
                assert_eq!(
                    headers,
                    vec!["Priority", "Name", "Age"],
                    "Should still have 3 columns"
                );
            }
            _ => panic!("Unexpected generation: {}", proxy.generation),
        },
        |proxy| {
            assert_eq!(proxy.generation, 1, "Expected exactly 1 generation");
        },
    );
}

#[test]
fn test_all_headers_includes_hidden() {
    // Test that all_headers() returns all columns including hidden ones
    test_hook(
        || {
            let trigger_name = use_signal(|| None);
            let trigger_age = use_signal(|| None);
            let col_name = TestColumn::name(trigger_name);
            let col_age = TestColumn::age(trigger_age);

            let context = TableContext::use_table_context((col_name, col_age));
            (context, trigger_age)
        },
        |(context, mut trigger_age), proxy| match proxy.generation {
            0 => {
                // Render headers to register use_effect
                for header in context.headers() {
                    let _ = header.render(vec![]);
                }

                // Initially, both columns are visible
                let visible_headers: Vec<_> = context.headers().collect();
                let all_headers: Vec<_> = context.all_headers().collect();
                assert_eq!(visible_headers.len(), 2, "Initially 2 visible columns");
                assert_eq!(all_headers.len(), 2, "Initially 2 total columns");

                // Hide Age column
                trigger_age.set(Some(ColumnAction::Hide));
            }
            1 => {
                // After hiding Age, headers() should return 1, all_headers() should return 2
                let visible_headers: Vec<_> = context.headers().map(|h| h.key()).collect();
                let all_headers: Vec<_> = context.all_headers().map(|h| h.key()).collect();

                assert_eq!(
                    visible_headers,
                    vec!["Name"],
                    "After hiding Age, only 1 visible column"
                );
                assert_eq!(
                    all_headers,
                    vec!["Name", "Age"],
                    "After hiding Age, all_headers() still returns 2 columns"
                );

                // Verify we can still access the hidden column via all_headers()
                let age_header = context
                    .all_headers()
                    .find(|h| h.key() == "Age")
                    .expect("Age header should be present");
                let age_ctx = age_header.column_context();
                assert!(!age_ctx.is_visible(), "Age column should be hidden");
            }
            _ => panic!("Unexpected generation: {}", proxy.generation),
        },
        |proxy| {
            assert_eq!(proxy.generation, 1, "Expected exactly 1 generation");
        },
    );
}

#[test]
fn test_column_context_hidden_column_sort() {
    // Test that hidden columns can still be used for sorting
    test_hook(
        || {
            let trigger_name = use_signal(|| None);
            let trigger_age = use_signal(|| None);
            let col_name = TestColumn::name(trigger_name);
            let col_age = TestColumn::age(trigger_age);

            let context = TableContext::use_table_context((col_name, col_age));
            let rows = use_signal(|| {
                vec![
                    TestPerson {
                        name: "Charlie".to_string(),
                        age: 35,
                    },
                    TestPerson {
                        name: "Bob".to_string(),
                        age: 25,
                    },
                    TestPerson {
                        name: "Alice".to_string(),
                        age: 30,
                    },
                ]
            });

            (context, rows, trigger_name, trigger_age)
        },
        |(context, rows, _trigger_name, mut trigger_age), proxy| match proxy.generation {
            0 => {
                // Render headers to register use_effect
                for header in context.headers() {
                    let _ = header.render(vec![]);
                }

                // Hide Age column and sort by it in one step
                trigger_age.set(Some(ColumnAction::Hide));
                trigger_age.set(Some(ColumnAction::Sort(SortGesture::AddFirst(Sort {
                    direction: SortDirection::Ascending,
                }))));
            }
            1 => {
                // Verify that sorting by hidden Age column works
                let sorted_names: Vec<String> = context
                    .rows(rows.into())
                    .map(|row_data| row_data.rows.read()[row_data.index].name.clone())
                    .collect();

                assert_eq!(
                    sorted_names,
                    vec![
                        "Bob".to_string(),     // age 25
                        "Alice".to_string(),   // age 30
                        "Charlie".to_string(), // age 35
                    ],
                    "Hidden Age column should still be sortable"
                );

                // Verify Age column is hidden
                let visible_headers: Vec<_> = context.headers().map(|h| h.key()).collect();
                assert_eq!(visible_headers, vec!["Name"], "Age column should be hidden, only Name visible");
            }
            _ => panic!("Unexpected generation: {}", proxy.generation),
        },
        |proxy| {
            assert_eq!(proxy.generation, 1, "Expected exactly 1 generation");
        },
    );
}

#[test]
fn test_column_context_swap_then_sort_hidden() {
    // Test complex scenario: swap columns, hide one, then sort by the hidden column
    test_hook(
        || {
            let trigger_name = use_signal(|| None);
            let trigger_age = use_signal(|| None);
            let col_name = TestColumn::name(trigger_name);
            let col_age = TestColumn::age(trigger_age);

            let context = TableContext::use_table_context((col_name, col_age));
            let rows = use_signal(|| {
                vec![
                    TestPerson {
                        name: "Charlie".to_string(),
                        age: 35,
                    },
                    TestPerson {
                        name: "Bob".to_string(),
                        age: 25,
                    },
                    TestPerson {
                        name: "Alice".to_string(),
                        age: 30,
                    },
                ]
            });

            (context, rows, trigger_name, trigger_age)
        },
        |(context, rows, mut trigger_name, _trigger_age), proxy| match proxy.generation {
            0 => {
                // Render headers to register use_effect
                for header in context.headers() {
                    let _ = header.render(vec![]);
                }

                // Swap, hide, and sort in one step
                trigger_name.set(Some(ColumnAction::SwapWith(1)));
                trigger_name.set(Some(ColumnAction::Hide));
                trigger_name.set(Some(ColumnAction::Sort(SortGesture::AddFirst(Sort {
                    direction: SortDirection::Ascending,
                }))));
            }
            1 => {
                // Verify that sorting by hidden Name column works
                let sorted_names: Vec<String> = context
                    .rows(rows.into())
                    .map(|row_data| row_data.rows.read()[row_data.index].name.clone())
                    .collect();

                assert_eq!(
                    sorted_names,
                    vec![
                        "Alice".to_string(),
                        "Bob".to_string(),
                        "Charlie".to_string(),
                    ],
                    "Hidden Name column should still be sortable"
                );

                // Verify only Age column is visible
                let visible_headers: Vec<_> = context.headers().collect();
                assert_eq!(
                    visible_headers.len(),
                    1,
                    "Should only have Age column visible"
                );
                assert_eq!(
                    visible_headers[0].key(),
                    "Age",
                    "Visible column should be Age (original index 1)"
                );
            }
            _ => panic!("Unexpected generation: {}", proxy.generation),
        },
        |proxy| {
            assert_eq!(proxy.generation, 1, "Expected exactly 1 generation");
        },
    );
}
