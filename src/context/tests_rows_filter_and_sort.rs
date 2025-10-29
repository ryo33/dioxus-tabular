use super::*;
use crate::test_suite::test_hook;
use crate::{GetRowData, TableColumn};
use std::cmp::Ordering;

// ==================== Test Data Structures ====================

#[derive(Clone, PartialEq, Debug)]
pub struct Person {
    pub name: String,
    pub age: u32,
}

impl Row for Person {
    fn key(&self) -> impl Into<String> {
        format!("{}_{}", self.name, self.age)
    }
}

#[derive(Clone, PartialEq)]
pub struct Name(pub String);

#[derive(Clone, PartialEq)]
pub struct Age(pub u32);

impl GetRowData<Name> for Person {
    fn get(&self) -> Name {
        Name(self.name.clone())
    }
}

impl GetRowData<Age> for Person {
    fn get(&self) -> Age {
        Age(self.age)
    }
}

// ==================== Filter Definitions ====================

#[derive(Clone, PartialEq, Debug)]
pub enum AgeFilter {
    MinAge(u32),
}

#[derive(Clone, PartialEq, Debug)]
#[allow(dead_code)]
pub enum NameFilter {
    StartsWith(String),
    Contains(String),
}

// ==================== Column Definitions ====================

#[derive(Clone, PartialEq)]
pub struct NameColumn {
    pub filter: Signal<Option<NameFilter>>,
}

impl NameColumn {
    pub fn use_column(filter: Option<NameFilter>) -> Self {
        Self {
            filter: use_signal(|| filter),
        }
    }
}

impl<R: Row + GetRowData<Name>> TableColumn<R> for NameColumn {
    fn column_name(&self) -> String {
        "name".into()
    }

    fn render_header(&self, _context: ColumnContext, _attributes: Vec<Attribute>) -> Element {
        rsx! {
            th {}
        }
    }

    fn render_cell(
        &self,
        _context: ColumnContext,
        _row: &R,
        _attributes: Vec<Attribute>,
    ) -> Element {
        rsx! {
            td {}
        }
    }

    fn filter(&self, row: &R) -> bool {
        match self.filter.read().as_ref() {
            None => true,
            Some(NameFilter::StartsWith(prefix)) => row.get().0.starts_with(prefix),
            Some(NameFilter::Contains(substring)) => row.get().0.contains(substring),
        }
    }

    fn compare(&self, a: &R, b: &R) -> Ordering {
        a.get().0.cmp(&b.get().0)
    }
}

#[derive(Clone, PartialEq)]
pub struct AgeColumn {
    pub filter: Signal<Option<AgeFilter>>,
}

impl AgeColumn {
    pub fn use_column(filter: Option<AgeFilter>) -> Self {
        Self {
            filter: use_signal(|| filter),
        }
    }
}

impl<R: Row + GetRowData<Age>> TableColumn<R> for AgeColumn {
    fn column_name(&self) -> String {
        "age".into()
    }

    fn render_header(&self, _context: ColumnContext, _attributes: Vec<Attribute>) -> Element {
        rsx! {
            th {}
        }
    }

    fn render_cell(
        &self,
        _context: ColumnContext,
        _row: &R,
        _attributes: Vec<Attribute>,
    ) -> Element {
        rsx! {
            td {}
        }
    }

    fn filter(&self, row: &R) -> bool {
        match self.filter.read().as_ref() {
            None => true,
            Some(AgeFilter::MinAge(min)) => row.get().0 >= *min,
        }
    }

    fn compare(&self, a: &R, b: &R) -> Ordering {
        a.get().0.cmp(&b.get().0)
    }
}

// ==================== Helper Functions ====================

fn ascending() -> Sort {
    Sort {
        direction: SortDirection::Ascending,
    }
}

fn descending() -> Sort {
    Sort {
        direction: SortDirection::Descending,
    }
}

/// Helper to extract the indices from the row iterator
fn collect_indices<C: Columns<Person>>(data: TableData<C, Person>) -> Vec<usize> {
    data.rows().map(|row| row.index).collect()
}

// ==================== Test Cases ====================

// A. Sort Only (No Filter)

#[test]
fn test_no_sort() {
    test_hook(
        || {
            let rows = use_signal(|| {
                vec![
                    Person {
                        name: "Charlie".to_string(),
                        age: 35,
                    },
                    Person {
                        name: "Alice".to_string(),
                        age: 30,
                    },
                    Person {
                        name: "Bob".to_string(),
                        age: 25,
                    },
                ]
            });
            let columns = (NameColumn::use_column(None), AgeColumn::use_column(None));
            let context = TableContext::use_table_context::<Person>(columns);
            let data = context.table_data(rows.into());
            (context, data)
        },
        |(_context, data), proxy| match proxy.generation {
            0 => {
                // No sort requested - should maintain original order
                let indices = collect_indices(data);
                assert_eq!(indices, vec![0, 1, 2]);
            }
            _ => panic!("Unexpected generation"),
        },
        |proxy| assert_eq!(proxy.generation, 1),
    );
}

#[test]
fn test_single_column_ascending() {
    test_hook(
        || {
            let rows = use_signal(|| {
                vec![
                    Person {
                        name: "Charlie".to_string(),
                        age: 35,
                    },
                    Person {
                        name: "Alice".to_string(),
                        age: 30,
                    },
                    Person {
                        name: "Bob".to_string(),
                        age: 25,
                    },
                ]
            });
            let columns = (NameColumn::use_column(None), AgeColumn::use_column(None));
            let context = TableContext::use_table_context::<Person>(columns);
            let data = context.table_data(rows.into());
            (context, data)
        },
        |(context, data), proxy| match proxy.generation {
            0 => {
                // Sort by name (column 0) ascending
                context
                    .data
                    .request_sort(0, SortGesture::AddFirst(ascending()));

                let indices = collect_indices(data);
                // Expected order: Alice(1), Bob(2), Charlie(0)
                assert_eq!(indices, vec![1, 2, 0]);
            }
            _ => panic!("Unexpected generation"),
        },
        |proxy| assert_eq!(proxy.generation, 1),
    );
}

#[test]
fn test_single_column_descending() {
    test_hook(
        || {
            let rows = use_signal(|| {
                vec![
                    Person {
                        name: "Alice".to_string(),
                        age: 25,
                    },
                    Person {
                        name: "Bob".to_string(),
                        age: 30,
                    },
                    Person {
                        name: "Charlie".to_string(),
                        age: 35,
                    },
                ]
            });
            let columns = (NameColumn::use_column(None), AgeColumn::use_column(None));
            let context = TableContext::use_table_context::<Person>(columns);
            let data = context.table_data(rows.into());
            (context, data)
        },
        |(context, data), proxy| match proxy.generation {
            0 => {
                // Sort by age (column 1) descending
                context
                    .data
                    .request_sort(1, SortGesture::AddFirst(descending()));

                let indices = collect_indices(data);
                // Expected order: Charlie(2:35), Bob(1:30), Alice(0:25)
                assert_eq!(indices, vec![2, 1, 0]);
            }
            _ => panic!("Unexpected generation"),
        },
        |proxy| assert_eq!(proxy.generation, 1),
    );
}

#[test]
fn test_multi_column_sort_priority() {
    test_hook(
        || {
            let rows = use_signal(|| {
                vec![
                    Person {
                        name: "Alice".to_string(),
                        age: 30,
                    },
                    Person {
                        name: "Bob".to_string(),
                        age: 25,
                    },
                    Person {
                        name: "Alice".to_string(),
                        age: 25,
                    },
                    Person {
                        name: "Bob".to_string(),
                        age: 30,
                    },
                ]
            });
            let columns = (NameColumn::use_column(None), AgeColumn::use_column(None));
            let context = TableContext::use_table_context::<Person>(columns);
            let data = context.table_data(rows.into());
            (context, data)
        },
        |(context, data), proxy| match proxy.generation {
            0 => {
                // Sort by name (column 0) first, then by age (column 1)
                // AddFirst reverses the order we need to add them
                context
                    .data
                    .request_sort(1, SortGesture::AddFirst(ascending()));
                context
                    .data
                    .request_sort(0, SortGesture::AddFirst(ascending()));

                let indices = collect_indices(data);
                // Expected order:
                // Alice,25 (2) - name:Alice, age:25
                // Alice,30 (0) - name:Alice, age:30
                // Bob,25   (1) - name:Bob, age:25
                // Bob,30   (3) - name:Bob, age:30
                assert_eq!(indices, vec![2, 0, 1, 3]);
            }
            _ => panic!("Unexpected generation"),
        },
        |proxy| assert_eq!(proxy.generation, 1),
    );
}

// B. Filter Only (No Sort)

#[test]
fn test_no_filter() {
    test_hook(
        || {
            let rows = use_signal(|| {
                vec![
                    Person {
                        name: "Alice".to_string(),
                        age: 30,
                    },
                    Person {
                        name: "Bob".to_string(),
                        age: 25,
                    },
                    Person {
                        name: "Charlie".to_string(),
                        age: 35,
                    },
                ]
            });
            // All rows pass filter
            let columns = (NameColumn::use_column(None), AgeColumn::use_column(None));
            let context = TableContext::use_table_context::<Person>(columns);
            let data = context.table_data(rows.into());
            (context, data)
        },
        |(_context, data), proxy| match proxy.generation {
            0 => {
                let indices = collect_indices(data);
                assert_eq!(indices, vec![0, 1, 2]);
            }
            _ => panic!("Unexpected generation"),
        },
        |proxy| assert_eq!(proxy.generation, 1),
    );
}

#[test]
fn test_partial_filter() {
    test_hook(
        || {
            let rows = use_signal(|| {
                vec![
                    Person {
                        name: "Bob".to_string(),
                        age: 25,
                    },
                    Person {
                        name: "Alice".to_string(),
                        age: 30,
                    },
                    Person {
                        name: "Charlie".to_string(),
                        age: 35,
                    },
                ]
            });
            // Only rows with age >= 30 pass filter
            let columns = (
                NameColumn::use_column(None),
                AgeColumn::use_column(Some(AgeFilter::MinAge(30))),
            );
            let context = TableContext::use_table_context::<Person>(columns);
            let data = context.table_data(rows.into());
            (context, data)
        },
        |(_context, data), proxy| match proxy.generation {
            0 => {
                let indices = collect_indices(data);
                // Expected: Alice(1:30), Charlie(2:35) - Bob(0:25) filtered out
                assert_eq!(indices, vec![1, 2]);
            }
            _ => panic!("Unexpected generation"),
        },
        |proxy| assert_eq!(proxy.generation, 1),
    );
}

#[test]
fn test_all_filtered_out() {
    test_hook(
        || {
            let rows = use_signal(|| {
                vec![
                    Person {
                        name: "Alice".to_string(),
                        age: 30,
                    },
                    Person {
                        name: "Bob".to_string(),
                        age: 25,
                    },
                    Person {
                        name: "Charlie".to_string(),
                        age: 35,
                    },
                ]
            });
            // No rows pass filter
            let columns = (
                NameColumn::use_column(None),
                AgeColumn::use_column(Some(AgeFilter::MinAge(50))),
            );
            let context = TableContext::use_table_context::<Person>(columns);
            let data = context.table_data(rows.into());
            (context, data)
        },
        |(_context, data), proxy| match proxy.generation {
            0 => {
                let indices = collect_indices(data);
                assert_eq!(indices, Vec::<usize>::new());
            }
            _ => panic!("Unexpected generation"),
        },
        |proxy| assert_eq!(proxy.generation, 1),
    );
}

// C. Filter + Sort

#[test]
fn test_filter_with_multi_column_sort() {
    test_hook(
        || {
            let rows = use_signal(|| {
                vec![
                    Person {
                        name: "Alice".to_string(),
                        age: 30,
                    },
                    Person {
                        name: "Bob".to_string(),
                        age: 20,
                    },
                    Person {
                        name: "Charlie".to_string(),
                        age: 35,
                    },
                    Person {
                        name: "David".to_string(),
                        age: 25,
                    },
                    Person {
                        name: "Alice".to_string(),
                        age: 28,
                    },
                ]
            });
            // Filter: age >= 25, Sort: name asc, then age asc
            let columns = (
                NameColumn::use_column(None),
                AgeColumn::use_column(Some(AgeFilter::MinAge(25))),
            );
            let context = TableContext::use_table_context::<Person>(columns);
            let data = context.table_data(rows.into());
            (context, data)
        },
        |(context, data), proxy| match proxy.generation {
            0 => {
                // Sort by age first (will be secondary), then by name (will be primary)
                context
                    .data
                    .request_sort(1, SortGesture::AddFirst(ascending()));
                context
                    .data
                    .request_sort(0, SortGesture::AddFirst(ascending()));

                let indices = collect_indices(data);
                // After filter (age >= 25): Alice(0:30), Charlie(2:35), David(3:25), Alice(4:28)
                // After sort by name then age:
                // Alice,28 (4)
                // Alice,30 (0)
                // Charlie,35 (2)
                // David,25 (3)
                assert_eq!(indices, vec![4, 0, 2, 3]);
            }
            _ => panic!("Unexpected generation"),
        },
        |proxy| assert_eq!(proxy.generation, 1),
    );
}

#[test]
fn test_empty_dataset() {
    test_hook(
        || {
            let rows = use_signal(Vec::<Person>::new);
            let columns = (NameColumn::use_column(None), AgeColumn::use_column(None));
            let context = TableContext::use_table_context::<Person>(columns);
            let data = context.table_data(rows.into());
            (context, data)
        },
        |(context, data), proxy| match proxy.generation {
            0 => {
                // Sort request on empty dataset should not panic
                context
                    .data
                    .request_sort(0, SortGesture::AddFirst(ascending()));

                let indices = collect_indices(data);
                assert_eq!(indices, Vec::<usize>::new());
            }
            _ => panic!("Unexpected generation"),
        },
        |proxy| assert_eq!(proxy.generation, 1),
    );
}
