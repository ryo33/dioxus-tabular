use dioxus::prelude::*;
use dioxus_tabular::*;
use std::cmp::Ordering;

// ==================== Data Structures ====================

#[derive(Clone, PartialEq, Debug)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub priority: Priority,
    pub status: Status,
    pub days_until: i32,
}

#[derive(Clone, PartialEq, Debug, PartialOrd, Ord, Eq)]
pub enum Priority {
    Low,
    Medium,
    High,
}

impl Priority {
    fn as_str(&self) -> &str {
        match self {
            Priority::Low => "Low",
            Priority::Medium => "Medium",
            Priority::High => "High",
        }
    }

    fn color(&self) -> &str {
        match self {
            Priority::Low => "#6b7280",
            Priority::Medium => "#f59e0b",
            Priority::High => "#ef4444",
        }
    }
}

#[derive(Clone, PartialEq, Debug, PartialOrd, Ord, Eq)]
pub enum Status {
    Todo,
    InProgress,
    Done,
}

impl Status {
    fn as_str(&self) -> &str {
        match self {
            Status::Todo => "Todo",
            Status::InProgress => "In Progress",
            Status::Done => "Done",
        }
    }

    fn color(&self) -> &str {
        match self {
            Status::Todo => "#6b7280",
            Status::InProgress => "#3b82f6",
            Status::Done => "#10b981",
        }
    }
}

impl Row for Task {
    fn key(&self) -> impl Into<String> {
        format!("task-{}", self.id)
    }
}

// ==================== GetRowData Implementations ====================

#[derive(Clone, PartialEq)]
pub struct Title(pub String);

#[derive(Clone, PartialEq)]
pub struct TaskPriority(pub Priority);

#[derive(Clone, PartialEq)]
pub struct TaskStatus(pub Status);

#[derive(Clone, PartialEq)]
pub struct DaysUntil(pub i32);

impl GetRowData<Title> for Task {
    fn get(&self) -> Title {
        Title(self.title.clone())
    }
}

impl GetRowData<TaskPriority> for Task {
    fn get(&self) -> TaskPriority {
        TaskPriority(self.priority.clone())
    }
}

impl GetRowData<TaskStatus> for Task {
    fn get(&self) -> TaskStatus {
        TaskStatus(self.status.clone())
    }
}

impl GetRowData<DaysUntil> for Task {
    fn get(&self) -> DaysUntil {
        DaysUntil(self.days_until)
    }
}

// ==================== Filter Enums ====================

#[derive(Clone, PartialEq, Debug)]
pub enum TitleFilter {
    Contains(String),
}

#[derive(Clone, PartialEq, Debug)]
pub enum PriorityFilter {
    MinPriority(Priority),
}

#[derive(Clone, PartialEq, Debug)]
pub enum StatusFilter {
    NotStatus(Status),
    IsStatus(Status),
}

#[derive(Clone, PartialEq, Debug)]
pub enum DaysFilter {
    Within(i32),
}

// ==================== Column Implementations ====================

#[derive(Clone, PartialEq)]
pub struct TitleColumn {
    pub filter: Signal<Option<TitleFilter>>,
}

impl TitleColumn {
    pub fn use_column(filter: Option<TitleFilter>) -> Self {
        Self {
            filter: use_signal(|| filter),
        }
    }
}

impl<R: Row + GetRowData<Title>> TableColumn<R> for TitleColumn {
    fn column_name(&self) -> String {
        "title".into()
    }

    fn render_header(&self, context: ColumnContext, attributes: Vec<Attribute>) -> Element {
        let mut filter_signal = self.filter;

        rsx! {
            th {..attributes,
                SortButton { context, label: "Title".to_string() }
                input {
                    r#type: "text",
                    placeholder: "Search...",
                    style: "width: 90%; padding: 4px; font-size: 12px;",
                    oninput: move |e| {
                        let val = e.value();
                        if val.is_empty() {
                            filter_signal.set(None);
                        } else {
                            filter_signal.set(Some(TitleFilter::Contains(val)));
                        }
                    },
                }
            }
        }
    }

    fn render_cell(&self, _context: ColumnContext, row: &R, attributes: Vec<Attribute>) -> Element {
        rsx! {
            td { ..attributes,"{row.get().0}" }
        }
    }

    fn filter(&self, row: &R) -> bool {
        match self.filter.read().as_ref() {
            None => true,
            Some(TitleFilter::Contains(substring)) => row
                .get()
                .0
                .to_lowercase()
                .contains(&substring.to_lowercase()),
        }
    }

    fn compare(&self, a: &R, b: &R) -> Ordering {
        a.get().0.cmp(&b.get().0)
    }
}

impl<R: Row + GetRowData<Title>> SerializableColumn<R> for TitleColumn {
    fn serialize_cell(&self, row: &R) -> impl serde::Serialize + '_ {
        row.get().0
    }
}

#[derive(Clone, PartialEq)]
pub struct PriorityColumn {
    pub filter: Signal<Option<PriorityFilter>>,
}

impl PriorityColumn {
    pub fn use_column(filter: Option<PriorityFilter>) -> Self {
        Self {
            filter: use_signal(|| filter),
        }
    }
}

impl<R: Row + GetRowData<TaskPriority>> TableColumn<R> for PriorityColumn {
    fn column_name(&self) -> String {
        "priority".into()
    }

    fn render_header(&self, context: ColumnContext, attributes: Vec<Attribute>) -> Element {
        let mut filter_signal = self.filter;

        rsx! {
            th {..attributes,
                SortButton { context, label: "Priority".to_string() }
                select {
                    style: "width: 90%; padding: 4px; font-size: 12px;",
                    onchange: move |e| {
                        match e.value().as_str() {
                            "medium" => {
                                filter_signal.set(Some(PriorityFilter::MinPriority(Priority::Medium)))
                            }
                            "high" => {
                                filter_signal.set(Some(PriorityFilter::MinPriority(Priority::High)))
                            }
                            _ => filter_signal.set(None),
                        }
                    },
                    option { value: "all", "All" }
                    option { value: "medium", "≥ Medium" }
                    option { value: "high", "High" }
                }
            }
        }
    }

    fn render_cell(&self, _context: ColumnContext, row: &R, attributes: Vec<Attribute>) -> Element {
        let priority = row.get().0;
        let color = priority.color();
        rsx! {
            td {..attributes,
                span { style: "color: {color}; font-weight: bold;", "{priority.as_str()}" }
            }
        }
    }

    fn filter(&self, row: &R) -> bool {
        match self.filter.read().as_ref() {
            None => true,
            Some(PriorityFilter::MinPriority(min)) => row.get().0 >= *min,
        }
    }

    fn compare(&self, a: &R, b: &R) -> Ordering {
        a.get().0.cmp(&b.get().0)
    }
}

impl<R: Row + GetRowData<TaskPriority>> SerializableColumn<R> for PriorityColumn {
    fn serialize_cell(&self, row: &R) -> impl serde::Serialize + '_ {
        row.get().0.as_str().to_string()
    }
}

#[derive(Clone, PartialEq)]
pub struct StatusColumn {
    pub filter: Signal<Option<StatusFilter>>,
}

impl StatusColumn {
    pub fn use_column(filter: Option<StatusFilter>) -> Self {
        Self {
            filter: use_signal(|| filter),
        }
    }
}

impl<R: Row + GetRowData<TaskStatus>> TableColumn<R> for StatusColumn {
    fn column_name(&self) -> String {
        "status".into()
    }

    fn render_header(&self, context: ColumnContext, attributes: Vec<Attribute>) -> Element {
        let mut filter_signal = self.filter;

        rsx! {
            th {..attributes,
                SortButton { context, label: "Status".to_string() }
                select {
                    style: "width: 90%; padding: 4px; font-size: 12px;",
                    onchange: move |e| {
                        match e.value().as_str() {
                            "incomplete" => {
                                filter_signal.set(Some(StatusFilter::NotStatus(Status::Done)))
                            }
                            "complete" => filter_signal.set(Some(StatusFilter::IsStatus(Status::Done))),
                            _ => filter_signal.set(None),
                        }
                    },
                    option { value: "all", "All" }
                    option { value: "incomplete", "Incomplete" }
                    option { value: "complete", "Complete" }
                }
            }
        }
    }

    fn render_cell(&self, _context: ColumnContext, row: &R, attributes: Vec<Attribute>) -> Element {
        let status = row.get().0;
        let color = status.color();
        rsx! {
            td {..attributes,
                span { style: "color: {color}; font-weight: bold;", "{status.as_str()}" }
            }
        }
    }

    fn filter(&self, row: &R) -> bool {
        match self.filter.read().as_ref() {
            None => true,
            Some(StatusFilter::NotStatus(s)) => row.get().0 != *s,
            Some(StatusFilter::IsStatus(s)) => row.get().0 == *s,
        }
    }

    fn compare(&self, a: &R, b: &R) -> Ordering {
        a.get().0.cmp(&b.get().0)
    }
}

impl<R: Row + GetRowData<TaskStatus>> SerializableColumn<R> for StatusColumn {
    fn serialize_cell(&self, row: &R) -> impl serde::Serialize + '_ {
        row.get().0.as_str().to_string()
    }
}

#[derive(Clone, PartialEq)]
pub struct DaysColumn {
    pub filter: Signal<Option<DaysFilter>>,
}

impl DaysColumn {
    pub fn use_column(filter: Option<DaysFilter>) -> Self {
        Self {
            filter: use_signal(|| filter),
        }
    }
}

impl<R: Row + GetRowData<DaysUntil>> TableColumn<R> for DaysColumn {
    fn column_name(&self) -> String {
        "days_until".into()
    }

    fn render_header(&self, context: ColumnContext, attributes: Vec<Attribute>) -> Element {
        let mut filter_signal = self.filter;

        rsx! {
            th {..attributes,
                SortButton { context, label: "Days Until".to_string() }
                select {
                    style: "width: 90%; padding: 4px; font-size: 12px;",
                    onchange: move |e| {
                        match e.value().as_str() {
                            "today" => filter_signal.set(Some(DaysFilter::Within(0))),
                            "week" => filter_signal.set(Some(DaysFilter::Within(7))),
                            _ => filter_signal.set(None),
                        }
                    },
                    option { value: "all", "All" }
                    option { value: "today", "Today" }
                    option { value: "week", "Within 7 Days" }
                }
            }
        }
    }

    fn render_cell(&self, _context: ColumnContext, row: &R, attributes: Vec<Attribute>) -> Element {
        let days = row.get().0;
        let text = if days < 0 {
            format!("{} days ago", -days)
        } else if days == 0 {
            "Today".to_string()
        } else {
            format!("in {} days", days)
        };
        let color = if days < 0 {
            "#ef4444"
        } else if days == 0 {
            "#f59e0b"
        } else {
            "#6b7280"
        };
        rsx! {
            td {..attributes,
                span { style: "color: {color};", "{text}" }
            }
        }
    }

    fn filter(&self, row: &R) -> bool {
        match self.filter.read().as_ref() {
            None => true,
            Some(DaysFilter::Within(max_days)) => row.get().0 <= *max_days && row.get().0 >= 0,
        }
    }

    fn compare(&self, a: &R, b: &R) -> Ordering {
        a.get().0.cmp(&b.get().0)
    }
}

impl<R: Row + GetRowData<DaysUntil>> SerializableColumn<R> for DaysColumn {
    fn serialize_cell(&self, row: &R) -> impl serde::Serialize + '_ {
        row.get().0
    }
}

// ==================== CSV Exporter ====================

struct CsvExporter {
    output: String,
}

impl CsvExporter {
    fn new() -> Self {
        Self {
            output: String::new(),
        }
    }

    fn finish(self) -> String {
        self.output
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
        cell: impl serde::Serialize + 'a,
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

// ==================== Sort Button Component ====================

#[component]
fn SortButton(context: ColumnContext, label: String) -> Element {
    let sort_info = use_memo(move || context.sort_info());

    rsx! {
        div { style: "display: flex; align-items: center; gap: 5px;",
            button {
                style: "background: none; border: none; cursor: pointer; font-weight: bold; padding: 0; color: white;",
                onclick: move |_| {
                    context
                        .request_sort(
                            SortGesture::AddLast(Sort {
                                direction: SortDirection::Ascending,
                            }),
                        );
                },
                "{label}"
                if let Some(info) = sort_info() {
                    span { style: "font-size: 10px; margin-left: 2px;", " ({info.priority + 1})" }
                }
            }
            if let Some(info) = sort_info() {
                button {
                    style: "background: none; border: none; cursor: pointer; padding: 0; font-size: 14px; color: white;",
                    onclick: move |_| {
                        context.request_sort(SortGesture::Toggle);
                    },
                    match info.direction {
                        SortDirection::Ascending => "↑",
                        SortDirection::Descending => "↓",
                    }
                }
                button {
                    style: "background: none; border: none; cursor: pointer; padding: 0; font-size: 12px; color: #999; margin-left: 2px;",
                    title: "Remove sort",
                    onclick: move |_| {
                        context.request_sort(SortGesture::Cancel);
                    },
                    "×"
                }
            }
        }
    }
}

// ==================== Sample Data ====================

fn sample_tasks() -> Vec<Task> {
    vec![
        Task {
            id: 1,
            title: "Fix login bug".to_string(),
            priority: Priority::High,
            status: Status::InProgress,
            days_until: 1,
        },
        Task {
            id: 2,
            title: "Write documentation".to_string(),
            priority: Priority::Low,
            status: Status::Todo,
            days_until: 14,
        },
        Task {
            id: 3,
            title: "Deploy to production".to_string(),
            priority: Priority::High,
            status: Status::Done,
            days_until: -2,
        },
        Task {
            id: 4,
            title: "Code review PR #42".to_string(),
            priority: Priority::Medium,
            status: Status::Todo,
            days_until: 2,
        },
        Task {
            id: 5,
            title: "Update dependencies".to_string(),
            priority: Priority::Low,
            status: Status::Done,
            days_until: -5,
        },
        Task {
            id: 6,
            title: "Refactor auth module".to_string(),
            priority: Priority::Medium,
            status: Status::InProgress,
            days_until: 7,
        },
        Task {
            id: 7,
            title: "Add unit tests".to_string(),
            priority: Priority::Medium,
            status: Status::Todo,
            days_until: 5,
        },
        Task {
            id: 8,
            title: "Design new UI".to_string(),
            priority: Priority::High,
            status: Status::InProgress,
            days_until: 0,
        },
        Task {
            id: 9,
            title: "Database migration".to_string(),
            priority: Priority::High,
            status: Status::Todo,
            days_until: 3,
        },
        Task {
            id: 10,
            title: "Performance optimization".to_string(),
            priority: Priority::Medium,
            status: Status::Done,
            days_until: -1,
        },
        Task {
            id: 11,
            title: "Security audit".to_string(),
            priority: Priority::High,
            status: Status::Todo,
            days_until: 4,
        },
        Task {
            id: 12,
            title: "Customer feedback analysis".to_string(),
            priority: Priority::Low,
            status: Status::Todo,
            days_until: 21,
        },
    ]
}

// ==================== Main App ====================

fn app() -> Element {
    let rows = use_signal(sample_tasks);

    let columns = (
        TitleColumn::use_column(None),
        PriorityColumn::use_column(None),
        StatusColumn::use_column(None),
        DaysColumn::use_column(None),
    );

    let data = use_tabular(columns, rows.into());

    // Export handler
    let export_csv = move |_| {
        let mut exporter = CsvExporter::new();
        if let Ok(()) = data.serialize(&mut exporter) {
            let csv = exporter.finish();
            println!("CSV Export:\n{}", csv);
            // In a real app, you would download this or copy to clipboard
        }
    };

    rsx! {
        div { style: "font-family: sans-serif; max-width: 1200px; margin: 0 auto; padding: 20px;",
            h1 { "Task Manager Demo" }

            // Controls
            div { style: "margin: 20px 0; display: flex; gap: 20px; align-items: center; flex-wrap: wrap;",
                // Column Visibility Toggles
                div { style: "display: flex; gap: 10px; align-items: center;",
                    span { style: "font-weight: bold;", "Columns:" }
                    for header in data.context.all_headers::<Task>() {
                        {
                            let col_ctx = header.column_context();
                            rsx! {
                                label {
                                    key: "{header.key()}",
                                    style: "display: flex; align-items: center; gap: 5px; cursor: pointer;",
                                    input {
                                        r#type: "checkbox",
                                        checked: col_ctx.is_visible(),
                                        onchange: move |e| {
                                            if e.checked() {
                                                col_ctx.show(None);
                                            } else {
                                                col_ctx.hide();
                                            }
                                        },
                                    }
                                    "{header.key()}"
                                }
                            }
                        }
                    }
                }

                // Export Button
                button {
                    style: "padding: 8px 16px; background: #3b82f6; color: white; border: none; border-radius: 4px; cursor: pointer; font-weight: bold;",
                    onclick: export_csv,
                    "Export CSV"
                }
            }

            // Table
            div { style: "overflow-x: auto;",
                table { style: "width: 100%; border-collapse: collapse; background: white;",
                    thead {
                        tr { style: "background: #1f2937; color: white;",
                            TableHeaders { data }
                        }
                    }
                    tbody {
                        for row in data.rows() {
                            tr {
                                key: "{row.key()}",
                                style: "border-bottom: 1px solid #e5e7eb;",
                                TableCells { row }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    dioxus::launch(app);
}
