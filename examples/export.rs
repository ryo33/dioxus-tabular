use dioxus::prelude::*;
use dioxus_tabular::*;
use serde::Serialize;

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

impl<R: Row + GetRowData<Id>> SerializableColumn<R> for IdColumn {
    fn serialize_cell(&self, row: &R) -> impl Serialize + '_ {
        row.get().0
    }
}

#[derive(Clone, PartialEq)]
pub struct NameColumn {
    serialize: Signal<bool>,
}

impl<R: Row + GetRowData<Name>> TableColumn<R> for NameColumn {
    fn column_name(&self) -> String {
        "name".into()
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

impl<R: Row + GetRowData<Name>> SerializableColumn<R> for NameColumn {
    fn serialize_cell(&self, row: &R) -> impl Serialize + '_ {
        row.get().0
    }

    fn include_in_export(&self) -> bool {
        *self.serialize.read()
    }
}

struct CsvExporter {
    output: String,
}

impl CsvExporter {
    fn finish(&mut self) {
        self.output.push('\n');
    }
}

impl Exporter for CsvExporter {
    type Error = serde_json::Error;

    fn serialize_header(&mut self, col: usize, header: &str) -> Result<(), Self::Error> {
        if col != 0 {
            self.output.push(',');
        }
        self.output.push_str(&serde_json::to_string(header)?);
        Ok(())
    }

    fn serialize_cell<'a>(
        &mut self,
        _row: usize,
        col: usize,
        cell: impl Serialize + 'a,
    ) -> Result<(), Self::Error> {
        if col == 0 {
            self.output.push('\n');
        } else {
            self.output.push(',');
        }
        self.output.push_str(&serde_json::to_string(&cell)?);
        Ok(())
    }
}

#[component]
pub fn Table<R: Row, C: Columns<R> + SerializableColumns<R>>(
    rows: ReadSignal<Vec<R>>,
    columns: C,
) -> Element {
    let mut serialized = use_signal(String::new);
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
        button {
            onclick: move |_| {
                let mut exporter = CsvExporter {
                    output: String::new(),
                };
                data.serialize(&mut exporter).unwrap();
                exporter.finish();
                serialized.set(exporter.output);
            },
            "serialize"
        }
        pre { "{serialized()}" }
    }
}

fn app() -> Element {
    let mut serialize_name = use_signal(|| true);
    let rows = use_signal(|| {
        vec![
            RowData {
                id: Id("1".to_string()),
                name: Name("Ryo".to_string()),
            },
            RowData {
                id: Id("2".to_string()),
                name: Name("Dioxus".to_string()),
            },
        ]
    });
    rsx! {
        div {
            input {
                r#type: "checkbox",
                checked: serialize_name(),
                oninput: move |_| {
                    serialize_name.set(!serialize_name());
                },
            }
            "serialize name"
        }
        Table {
            rows,
            columns: (
                IdColumn,
                NameColumn {
                    serialize: serialize_name,
                },
            ),
        }
    }
}

fn main() {
    dioxus::launch(app);
}
