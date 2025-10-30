# dioxus-tabular

[![GitHub](https://img.shields.io/badge/GitHub-ryo33/dioxus--tabular-222222)](https://github.com/ryo33/dioxus-tabular)
![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)
[![Crates.io](https://img.shields.io/crates/v/dioxus-tabular)](https://crates.io/crates/dioxus-tabular)
[![docs.rs](https://img.shields.io/docsrs/dioxus-tabular)](https://docs.rs/dioxus-tabular)
![GitHub Repo stars](https://img.shields.io/github/stars/ryo33/dioxus-tabular?style=social)

**Type-safe and composable table framework for Dioxus.**
Build reactive tabular UIs with declarative column definitions and multi-column sorting.

## Version compatibility

```toml
# for Dioxus 0.6.3
dioxus-tabular = "0.1"

# for Dioxus 0.7.0-rc.3
dioxus-tabular = { git = "https://github.com/ryo33/dioxus-tabular", branch = "dioxus-0.7" }
```

## Overview

`dioxus-tabular` is a Dioxus-native framework for building **structured, declarative, and strongly-typed table UIs**.

Instead of configuring your table with dynamic descriptors or ad-hoc data models, you define **columns as typed components**.
Each column owns its own render logic, filter behavior, and sort comparator — and can hold local reactive state via `Signal`.

This approach may feel a little more verbose at first, but it unlocks:

- **Composable, type-safe column definitions**
- **Declarative multi-column sorting and filtering**
- **Centralized column reordering and visibility control**
- **Extensible abstractions** (`Row`, `GetRowData`, `TableColumn`)
- **Easy export to various formats** (CSV, Excel, etc.)

## Design Philosophy

- Columns are *first-class citizens*: each is a self-contained logic unit.
- Columns are *composable*: they can be freely combined into tables.
- Columns are *type-safe*: If a column type does not fit to the row data type, the compiler will complain.
- Columns are *self-contained*: they can hold their own state, filtering logic, sorting logic, and rendering logic.
- All columns, data, and tables are *reusable* and *swappable*: All of them does not depend on each other, so you can mix and match them as you like.

## Core Concepts

| Trait / Struct  | Description                                                        |
| --------------- | ------------------------------------------------------------------ |
| `Row`           | Defines the unique key and identity of each row.                   |
| `GetRowData<T>` | Provides access to the data of the row by a specific type.         |
| `TableColumn`   | Describes how a single column renders, filters, and compares rows. |
| `Columns`       | A composed collection of `TableColumn`s, implemented for tuples.   |

## Features

### Multi-Column Sorting

Tables support declarative multi-column sorting with priority control:

- Each column can define its own comparison logic via `TableColumn::compare()`
- Each column can request:
  - sort direction (ascending or descending) or toggle the direction
  - sort priority (primary or last)
  - sort removal
- All sort requests are applied automatically when rendering rows

### Row Filtering

Columns can implement custom filtering logic:

- Each column defines its own `TableColumn::filter()` method
- All filters from all columns are applied automatically when rendering rows

### Column Ordering and Visibility

Control which columns are displayed and in what order:

- **Hide/Show**: `hide_column()`, `show_column()` - Toggle column visibility
- **Reorder**: `move_to()`, `swap_columns()` - Change column positions
- **Navigate**: `move_forward()`, `move_backward()` - Move columns incrementally
- **Reset**: `reset_column_order()` - Restore default order and visibility

Access these methods through `TableContextData` or `ColumnContext`.

### Export to various formats (Needs optional `export` feature)

You can export table data with your custom exporter implementation. Enable the `export` feature, and implement the `SerializableColumn` trait for your columns and the `Exporter` trait for your exporter.

See the [example](examples/export.rs) for more details.

## Example scenario

You could define the following types and implement those traits like the following example:

Rows:

- `User` implements `Row`, `GetRowData<UserId>` and `GetRowData<UserName>`
- `AccessLog` implements `Row`, `GetRowData<AccessLogId>`, `GetRowData<Timestamp>` and `GetRowData<UserId>`

Columns:

- `UserIdColumn` implements `TableColumn<T>` for every `T where GetRowData<UserId>`
- `UserNameColumn` implements `TableColumn<T>` for every `T where GetRowData<UserName>`
- `AccessLogIdColumn` implements `TableColumn<T>` for every `T where GetRowData<AccessLogId>`
- `TimestampColumn` implements `TableColumn<T>` for every `T where GetRowData<Timestamp>`

And, you define a simple table component like the following:

```rust
#[component]
pub fn SimpleTable<R: Row, C: Columns<R>>(rows: ReadOnlySignal<Vec<R>>, columns: C) -> Element {
    let table_context = TableContext::use_table_context(columns.column_names());
    rsx! {
        table {
            thead {
                tr { {columns.render_headers(table_context)} }
            }
            tbody {
                for row in rows.iter() {
                    tr { key: "{row.key().into()}",
                        {columns.render_columns(table_context, &row, vec![])}
                    }
                }
            }
        }
    }
}
```

and another one:

```rust
#[component]
pub fn FancyTable<R: Row, C: Columns<R>>(rows: ReadOnlySignal<Vec<R>>, columns: C) -> Element {
    // Another table component with different styling or features than the above one.
    // ...
}
```

Now you can render many kinds of tables with different column combinations like the following:

```rust
let users: Vec<User> = ...;
let access_logs: Vec<AccessLog> = ...;
rsx! {
    // Simple table with two columns for showing user ids and names.
    SimpleTable { rows: users, columns: (UserIdColumn, UserNameColumn) }
    // Same data and columns, but with different styling or features than the above one.
    FancyTable { rows: users, columns: (UserIdColumn, UserNameColumn) }
    // Table for accces logs. Notice that the UserIdColumn is reusable for both users and access logs.
    SimpleTable { rows: access_logs, columns: (AccessLogIdColumn, TimestampColumn, UserIdColumn) }
}
```

Also, if you implement filtering and sorting logic in the columns, any tables have sorting and filtering features without any additional code to each table.

## License

MIT or Apache 2.0 at your option.
