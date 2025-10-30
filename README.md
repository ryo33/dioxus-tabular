# dioxus-tabular

[![GitHub](https://img.shields.io/badge/GitHub-ryo33/dioxus--tabular-222222)](https://github.com/ryo33/dioxus-tabular)
![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)
[![Crates.io](https://img.shields.io/crates/v/dioxus-tabular)](https://crates.io/crates/dioxus-tabular)
[![docs.rs](https://img.shields.io/docsrs/dioxus-tabular)](https://docs.rs/dioxus-tabular)
![GitHub Repo stars](https://img.shields.io/github/stars/ryo33/dioxus-tabular?style=social)

**Type-safe and composable table framework for Dioxus.**
Define self-contained columns with rendering, filtering, and sorting logic that work independently of table data types and components.

## Version compatibility

```toml
# for Dioxus 0.6.3
dioxus-tabular = "0.1"

# for Dioxus 0.7.0-rc.3
dioxus-tabular = { git = "https://github.com/ryo33/dioxus-tabular", branch = "dioxus-0.7" }
```

## Overview

`dioxus-tabular` is a Dioxus-native framework for building **structured, declarative, and strongly-typed table UIs**.

Instead of configuring your table with dynamic descriptors or ad-hoc data models, you define **columns as typed components** that don't depend on the actual table data type or table component.
Each column owns its own rendering, filtering, and sorting logic — and can hold local reactive state via `Signal`.

This approach may seem more verbose initially, but it enables:

- **Self-contained, type-safe column definitions** that work with any compatible row type
- **Interchangeable columns, table data types, and table components**
- **Declarative multi-column sorting and filtering**
- **Column reordering, hiding, and visibility control**
- **Table export** to various formats (CSV, Excel, etc.)
- **Extensible abstractions** (`Row`, `GetRowData`, `TableColumn`)

## Design Philosophy

- Columns are *first-class citizens*: each is a self-contained logic unit.
- Columns are *composable*: they can be freely combined into tables.
- Columns are *type-safe*: If a column type does not fit the row data type, the compiler will complain.
- Columns are *independent*: they define their own state, filtering, sorting, and rendering logic.
- Columns, data, and tables are *reusable* and *interchangeable*: They don't depend on each other, so you can freely mix and match them.

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
- All column filters are automatically applied when rendering rows

### Column Ordering and Visibility

Control which columns are displayed and in what order:

- **Hide/Show**: `hide_column()`, `show_column()` - Toggle column visibility
- **Reorder**: `move_to()`, `swap_columns()` - Change column positions
- **Navigate**: `move_forward()`, `move_backward()` - Move columns incrementally
- **Reset**: `reset_column_order()` - Restore default order and visibility

Access these methods through `TableContextData` or `ColumnContext`.

### Export to various formats (requires the optional `export` feature)

You can export table data with your custom exporter implementation. Enable the `export` feature, and implement the `SerializableColumn` trait for your columns and the `Exporter` trait for your exporter.

See the [example](examples/export.rs) for more details.

## Example scenario

You can define types and implement traits as follows:

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

You can then render various kinds of tables with different column combinations:

```rust
let users: Vec<User> = ...;
let access_logs: Vec<AccessLog> = ...;
rsx! {
    // Simple table with two columns for showing user ids and names.
    SimpleTable { rows: users, columns: (UserIdColumn, UserNameColumn) }
    // Same data and columns, but with different styling or features than the above one.
    FancyTable { rows: users, columns: (UserIdColumn, UserNameColumn) }
    // Table for access logs. The UserIdColumn is reusable across different table types.
    SimpleTable { rows: access_logs, columns: (AccessLogIdColumn, TimestampColumn, UserIdColumn) }
}
```

Also, if you implement filtering and sorting logic in the columns, any tables have sorting and filtering features without any additional code to each table.

## License

MIT or Apache 2.0 at your option.
