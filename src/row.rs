pub trait Row: PartialEq + Clone + 'static {
    fn key(&self) -> impl Into<String>;
}

pub trait GetRowData<T> {
    fn get(&self) -> T;
}
