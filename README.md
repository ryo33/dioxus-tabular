# dioxus-tabular

**Type-safe and composable table framework for Dioxus.**
Build reactive tabular UIs with declarative column definitions and multi-column sorting.

## Overview

`dioxus-tabular` is a Dioxus-native framework for building **structured, declarative, and strongly-typed table UIs**.

Instead of configuring your table with dynamic descriptors or ad-hoc data models, you define **columns as typed components**.
Each column owns its own render logic, filter behavior, and sort comparator — and can hold local reactive state via `Signal`.

This approach may feel a little more verbose at first, but it unlocks:

- **Composable, type-safe column definitions**
- **Fine-grained reactivity** (each column can hold its own state)
- **Declarative multi-column sorting**
- **Extensible abstractions** (`Row`, `GetRowData`, `TableColumn`)
- **Full Dioxus integration**: built with `rsx!` and `Signal` from the ground up

## Design Philosophy

- Columns are *first-class citizens*: each is a self-contained logic unit.
- Columns are *reusable*: they can be used with any type of data types that implements the accessors trait `GetColumn`.
- Columns are *composable*: they can be freely combined into tables.
- Columns are *type-safe*: If a column type does not fit to the row data type, the compiler will complain.
- Columns are *self-contained*: they can hold their own state, filtering logic, sorting logic, and rendering logic.

## Core Concepts

| Trait / Struct  | Description                                                        |
| --------------- | ------------------------------------------------------------------ |
| `Row`           | Defines the unique key and identity of each row.                   |
| `GetRowData<T>` | Provides access to the data of the row by a specific type.         |
| `TableColumn`   | Describes how a single column renders, filters, and compares rows. |
| `Columns`       | A composed collection of `TableColumn`s, implemented for tuples.   |
| `TableContext`  | Holds tables state                                                 |

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
pub fn Table<R: Row, C: Columns<R>>(rows: ReadOnlySignal<Vec<R>>, columns: C) -> Element {
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

Now you can render many kinds of tables with different column combinations like the following:

```rust
let users: Vec<User> = ...;
let access_logs: Vec<AccessLog> = ...;
rsx! {
    Table { rows: users, columns: (UserIdColumn, UserNameColumn) }
    Table { rows: access_logs, columns: (AccessLogIdColumn, TimestampColumn, UserIdColumn) }
    Table { rows: access_logs, columns: UserIdColumn }
}
```

Also, if you implement filtering and sorting logic in the columns, any tables have sorting and filtering features without any additional code to each table.

## License

MIT or Apache 2.0 at your option.
