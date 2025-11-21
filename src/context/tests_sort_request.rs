use super::*;
use crate::test_suite::test_hook;

/// Helper to create a Sort with Ascending direction
fn ascending() -> Sort {
    Sort {
        direction: SortDirection::Ascending,
    }
}

/// Helper to create a Sort with Descending direction
fn descending() -> Sort {
    Sort {
        direction: SortDirection::Descending,
    }
}

#[test]
fn test_cancel_on_empty_list() {
    test_hook(
        || TableContextData {
            sorts: use_signal(Vec::new),
            column_names: use_signal(|| {
                vec![
                    "Column 0".to_string(),
                    "Column 1".to_string(),
                    "Column 2".to_string(),
                ]
            }),
            column_order: use_signal(|| ColumnOrder::new(3)),
        },
        |context, proxy| match proxy.generation {
            0 => {
                // Cancel on empty list
                context.request_sort(0, SortGesture::Cancel);

                // Verify list is still empty
                assert_eq!(context.sorts.read().len(), 0);
            }
            _ => panic!("Unexpected generation"),
        },
        |proxy| assert_eq!(proxy.generation, 1),
    );
}

#[test]
fn test_cancel_removes_existing_sort() {
    test_hook(
        || TableContextData {
            sorts: use_signal(Vec::new),
            column_names: use_signal(|| {
                vec![
                    "Column 0".to_string(),
                    "Column 1".to_string(),
                    "Column 2".to_string(),
                ]
            }),
            column_order: use_signal(|| ColumnOrder::new(3)),
        },
        |context, proxy| match proxy.generation {
            0 => {
                // Setup: Add a sort on column 0
                context.request_sort(0, SortGesture::AddFirst(ascending()));
                assert_eq!(context.sorts.read().len(), 1);

                // Cancel the sort on column 0
                context.request_sort(0, SortGesture::Cancel);

                // Verify it's removed
                assert_eq!(context.sorts.read().len(), 0);
            }
            1 | 2 => {
                // Rerender after signal changes - no action needed
            }
            _ => panic!("Unexpected generation: {}", proxy.generation),
        },
        |proxy| {
            assert!(proxy.generation >= 1, "Expected at least one rerender");
        },
    );
}

#[test]
fn test_cancel_on_column_without_sort() {
    test_hook(
        || TableContextData {
            sorts: use_signal(Vec::new),
            column_names: use_signal(|| {
                vec![
                    "Column 0".to_string(),
                    "Column 1".to_string(),
                    "Column 2".to_string(),
                ]
            }),
            column_order: use_signal(|| ColumnOrder::new(3)),
        },
        |context, proxy| match proxy.generation {
            0 => {
                // Setup: Add a sort on column 1
                context.request_sort(1, SortGesture::AddFirst(ascending()));
                assert_eq!(context.sorts.read().len(), 1);

                // Cancel on column 0 (which has no sort)
                context.request_sort(0, SortGesture::Cancel);

                // Verify the list is unchanged
                let sorts = context.sorts.read();
                assert_eq!(sorts.len(), 1);
                assert_eq!(sorts[0].column, 1);
            }
            1 => {
                // Rerender after signal changes - no action needed
            }
            _ => panic!("Unexpected generation: {}", proxy.generation),
        },
        |proxy| {
            assert!(proxy.generation >= 1, "Expected at least one rerender");
        },
    );
}

#[test]
fn test_cancel_preserves_other_column_sorts() {
    test_hook(
        || TableContextData {
            sorts: use_signal(Vec::new),
            column_names: use_signal(|| {
                vec![
                    "Column 0".to_string(),
                    "Column 1".to_string(),
                    "Column 2".to_string(),
                ]
            }),
            column_order: use_signal(|| ColumnOrder::new(3)),
        },
        |context, proxy| match proxy.generation {
            0 => {
                // Setup: Add sorts on columns 0, 1, 2
                context.request_sort(0, SortGesture::AddFirst(ascending()));
                context.request_sort(1, SortGesture::AddLast(descending()));
                context.request_sort(2, SortGesture::AddLast(ascending()));
                assert_eq!(context.sorts.read().len(), 3);

                // Cancel column 1
                context.request_sort(1, SortGesture::Cancel);

                // Verify only column 1 is removed, others preserved
                let sorts = context.sorts.read();
                assert_eq!(sorts.len(), 2);
                assert_eq!(sorts[0].column, 0);
                assert_eq!(sorts[1].column, 2);
            }
            1..=4 => {
                // Rerender after signal changes - no action needed
            }
            _ => panic!("Unexpected generation: {}", proxy.generation),
        },
        |proxy| {
            assert!(proxy.generation >= 1, "Expected at least one rerender");
        },
    );
}

#[test]
fn test_add_first_ascending_on_empty_list() {
    test_hook(
        || TableContextData {
            sorts: use_signal(Vec::new),
            column_names: use_signal(|| {
                vec![
                    "Column 0".to_string(),
                    "Column 1".to_string(),
                    "Column 2".to_string(),
                ]
            }),
            column_order: use_signal(|| ColumnOrder::new(3)),
        },
        |context, proxy| match proxy.generation {
            0 => {
                // Add first sort with ascending direction
                context.request_sort(0, SortGesture::AddFirst(ascending()));

                // Verify added at position 0
                let sorts = context.sorts.read();
                assert_eq!(sorts.len(), 1);
                assert_eq!(sorts[0].column, 0);
                assert_eq!(sorts[0].sort.direction, SortDirection::Ascending);
            }
            _ => panic!("Unexpected generation"),
        },
        |proxy| assert_eq!(proxy.generation, 1),
    );
}

#[test]
fn test_add_first_replaces_existing_sort_on_same_column() {
    test_hook(
        || TableContextData {
            sorts: use_signal(Vec::new),
            column_names: use_signal(|| {
                vec![
                    "Column 0".to_string(),
                    "Column 1".to_string(),
                    "Column 2".to_string(),
                ]
            }),
            column_order: use_signal(|| ColumnOrder::new(3)),
        },
        |context, proxy| match proxy.generation {
            0 => {
                // Setup: Add ascending sort on column 0
                context.request_sort(0, SortGesture::AddFirst(ascending()));
                assert_eq!(context.sorts.read().len(), 1);

                // Replace with descending sort on same column
                context.request_sort(0, SortGesture::AddFirst(descending()));

                // Verify replaced (still only 1 sort)
                let sorts = context.sorts.read();
                assert_eq!(sorts.len(), 1);
                assert_eq!(sorts[0].column, 0);
                assert_eq!(sorts[0].sort.direction, SortDirection::Descending);
            }
            1 | 2 => {
                // Rerender after signal changes - no action needed
            }
            _ => panic!("Unexpected generation: {}", proxy.generation),
        },
        |proxy| {
            assert!(proxy.generation >= 1, "Expected at least one rerender");
        },
    );
}

#[test]
fn test_add_first_with_multiple_columns_sorted() {
    test_hook(
        || TableContextData {
            sorts: use_signal(Vec::new),
            column_names: use_signal(|| {
                vec![
                    "Column 0".to_string(),
                    "Column 1".to_string(),
                    "Column 2".to_string(),
                ]
            }),
            column_order: use_signal(|| ColumnOrder::new(3)),
        },
        |context, proxy| match proxy.generation {
            0 => {
                // Setup: Add sorts on columns 1 and 2
                context.request_sort(1, SortGesture::AddFirst(ascending()));
                context.request_sort(2, SortGesture::AddLast(descending()));

                // Add first sort on column 0
                context.request_sort(0, SortGesture::AddFirst(ascending()));

                // Verify column 0 is first, others shifted
                let sorts = context.sorts.read();
                assert_eq!(sorts.len(), 3);
                assert_eq!(sorts[0].column, 0);
                assert_eq!(sorts[1].column, 1);
                assert_eq!(sorts[2].column, 2);
            }
            _ => panic!("Unexpected generation"),
        },
        |proxy| assert_eq!(proxy.generation, 1),
    );
}

#[test]
fn test_add_first_moves_column_from_last_to_first() {
    test_hook(
        || TableContextData {
            sorts: use_signal(Vec::new),
            column_names: use_signal(|| {
                vec![
                    "Column 0".to_string(),
                    "Column 1".to_string(),
                    "Column 2".to_string(),
                ]
            }),
            column_order: use_signal(|| ColumnOrder::new(3)),
        },
        |context, proxy| match proxy.generation {
            0 => {
                // Setup: Add sorts on columns 0, 1, 2
                context.request_sort(0, SortGesture::AddFirst(ascending()));
                context.request_sort(1, SortGesture::AddLast(ascending()));
                context.request_sort(2, SortGesture::AddLast(ascending()));

                // Move column 2 to first
                context.request_sort(2, SortGesture::AddFirst(descending()));

                // Verify column 2 is now first
                let sorts = context.sorts.read();
                assert_eq!(sorts.len(), 3);
                assert_eq!(sorts[0].column, 2);
                assert_eq!(sorts[0].sort.direction, SortDirection::Descending);
                assert_eq!(sorts[1].column, 0);
                assert_eq!(sorts[2].column, 1);
            }
            _ => panic!("Unexpected generation"),
        },
        |proxy| assert_eq!(proxy.generation, 1),
    );
}

#[test]
fn test_add_last_ascending_on_empty_list() {
    test_hook(
        || TableContextData {
            sorts: use_signal(Vec::new),
            column_names: use_signal(|| {
                vec![
                    "Column 0".to_string(),
                    "Column 1".to_string(),
                    "Column 2".to_string(),
                ]
            }),
            column_order: use_signal(|| ColumnOrder::new(3)),
        },
        |context, proxy| match proxy.generation {
            0 => {
                // Add last sort with ascending direction on empty list
                context.request_sort(0, SortGesture::AddLast(ascending()));

                // Verify added at position 0 (which is also the end)
                let sorts = context.sorts.read();
                assert_eq!(sorts.len(), 1);
                assert_eq!(sorts[0].column, 0);
                assert_eq!(sorts[0].sort.direction, SortDirection::Ascending);
            }
            _ => panic!("Unexpected generation"),
        },
        |proxy| assert_eq!(proxy.generation, 1),
    );
}

#[test]
fn test_add_last_replaces_existing_sort_on_same_column() {
    test_hook(
        || TableContextData {
            sorts: use_signal(Vec::new),
            column_names: use_signal(|| {
                vec![
                    "Column 0".to_string(),
                    "Column 1".to_string(),
                    "Column 2".to_string(),
                ]
            }),
            column_order: use_signal(|| ColumnOrder::new(3)),
        },
        |context, proxy| match proxy.generation {
            0 => {
                // Setup: Add ascending sort on column 0
                context.request_sort(0, SortGesture::AddLast(ascending()));
                assert_eq!(context.sorts.read().len(), 1);

                // Replace with descending sort on same column
                context.request_sort(0, SortGesture::AddLast(descending()));

                // Verify replaced (still only 1 sort)
                let sorts = context.sorts.read();
                assert_eq!(sorts.len(), 1);
                assert_eq!(sorts[0].column, 0);
                assert_eq!(sorts[0].sort.direction, SortDirection::Descending);
            }
            1 | 2 => {
                // Rerender after signal changes - no action needed
            }
            _ => panic!("Unexpected generation: {}", proxy.generation),
        },
        |proxy| {
            assert!(proxy.generation >= 1, "Expected at least one rerender");
        },
    );
}

#[test]
fn test_add_last_with_multiple_columns_sorted() {
    test_hook(
        || TableContextData {
            sorts: use_signal(Vec::new),
            column_names: use_signal(|| {
                vec![
                    "Column 0".to_string(),
                    "Column 1".to_string(),
                    "Column 2".to_string(),
                ]
            }),
            column_order: use_signal(|| ColumnOrder::new(3)),
        },
        |context, proxy| match proxy.generation {
            0 => {
                // Setup: Add sorts on columns 0 and 1
                context.request_sort(0, SortGesture::AddFirst(ascending()));
                context.request_sort(1, SortGesture::AddLast(ascending()));

                // Add last sort on column 2
                context.request_sort(2, SortGesture::AddLast(descending()));

                // Verify column 2 is last
                let sorts = context.sorts.read();
                assert_eq!(sorts.len(), 3);
                assert_eq!(sorts[0].column, 0);
                assert_eq!(sorts[1].column, 1);
                assert_eq!(sorts[2].column, 2);
                assert_eq!(sorts[2].sort.direction, SortDirection::Descending);
            }
            _ => panic!("Unexpected generation"),
        },
        |proxy| assert_eq!(proxy.generation, 1),
    );
}

#[test]
fn test_add_last_moves_column_from_first_to_last() {
    test_hook(
        || TableContextData {
            sorts: use_signal(Vec::new),
            column_names: use_signal(|| {
                vec![
                    "Column 0".to_string(),
                    "Column 1".to_string(),
                    "Column 2".to_string(),
                ]
            }),
            column_order: use_signal(|| ColumnOrder::new(3)),
        },
        |context, proxy| match proxy.generation {
            0 => {
                // Setup: Add sorts on columns 0, 1, 2
                context.request_sort(0, SortGesture::AddFirst(ascending()));
                context.request_sort(1, SortGesture::AddLast(ascending()));
                context.request_sort(2, SortGesture::AddLast(ascending()));

                // Move column 0 to last
                context.request_sort(0, SortGesture::AddLast(descending()));

                // Verify column 0 is now last
                let sorts = context.sorts.read();
                assert_eq!(sorts.len(), 3);
                assert_eq!(sorts[0].column, 1);
                assert_eq!(sorts[1].column, 2);
                assert_eq!(sorts[2].column, 0);
                assert_eq!(sorts[2].sort.direction, SortDirection::Descending);
            }
            _ => panic!("Unexpected generation"),
        },
        |proxy| assert_eq!(proxy.generation, 1),
    );
}
