use dioxus::prelude::*;
use dioxus_tabular::*;

#[derive(Clone, PartialEq)]
pub struct RowData {
    pub id: Id,
    pub name: Name,
}

#[derive(Clone, PartialEq)]
pub struct Id(pub String);

#[derive(Clone, PartialEq)]
pub struct Name(pub String);

impl Row for RowData {
    fn key(&self) -> impl Into<String> {
        self.id.0.clone()
    }
}

impl GetRowData<Id> for RowData {
    fn get(&self) -> Id {
        self.id.clone()
    }
}

impl GetRowData<Name> for RowData {
    fn get(&self) -> Name {
        self.name.clone()
    }
}

#[derive(Clone, PartialEq)]
pub struct IdColumn;

impl<R: Row + GetRowData<Id>> TableColumn<R> for IdColumn {
    fn column_name(&self) -> String {
        "id".into()
    }

    fn serialize(&self, row: &R) -> String {
        row.get().0
    }

    fn render_header(&self, _context: ColumnContext, attributes: Vec<Attribute>) -> Element {
        rsx! {
            th { ..attributes,"ID" }
        }
    }
    fn render_cell(&self, _context: ColumnContext, row: &R, attributes: Vec<Attribute>) -> Element {
        rsx! {
            td { ..attributes,"{row.get().0}" }
        }
    }

    fn filter(&self, _row: &R) -> bool {
        true
    }

    fn compare(&self, a: &R, b: &R) -> std::cmp::Ordering {
        a.get().0.cmp(&b.get().0)
    }
}

#[derive(Clone, PartialEq)]
pub struct NameColumn;

impl<R: Row + GetRowData<Name>> TableColumn<R> for NameColumn {
    fn column_name(&self) -> String {
        "name".into()
    }
    fn serialize(&self, row: &R) -> String {
        row.get().0
    }
    fn render_header(&self, _context: ColumnContext, attributes: Vec<Attribute>) -> Element {
        rsx! {
            th { ..attributes,"Name" }
        }
    }
    fn render_cell(&self, _context: ColumnContext, row: &R, attributes: Vec<Attribute>) -> Element {
        rsx! {
            td { ..attributes,"{row.get().0}" }
        }
    }
    fn filter(&self, _row: &R) -> bool {
        true
    }
    fn compare(&self, a: &R, b: &R) -> std::cmp::Ordering {
        a.get().0.cmp(&b.get().0)
    }
}

#[component]
pub fn Table<R: Row, C: Columns<R>>(rows: ReadOnlySignal<Vec<R>>, columns: C) -> Element {
    let data = use_tabular(columns, rows);
    rsx! {
        table {
            thead {
                tr {
                    TableHeaders { data }
                }
            }
            tbody {
                for row in data.rows() {
                    tr { key: "{row.key()}",
                        TableCells { row }
                    }
                }
            }
        }
    }
}

fn app() -> Element {
    let rows = use_signal(|| {
        vec![
            RowData {
                id: Id("1".to_string()),
                name: Name("John Doe".to_string()),
            },
            RowData {
                id: Id("2".to_string()),
                name: Name("Jane Doe".to_string()),
            },
        ]
    });
    rsx! {
        Table { rows, columns: (IdColumn, NameColumn) }
        Table { rows, columns: (NameColumn, IdColumn) }
    }
}

fn main() {
    dioxus::launch(app);
}
