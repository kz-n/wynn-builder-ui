pub struct DBReader {
    pub db: sqlite::Connection,
    pub table_name: String,
    pub columns: Vec<String>,
    pub rows: Vec<Vec<String>>,
    pub query: String,
}

impl DBReader {
    pub fn new(db: sqlite::Connection, table_name: String) -> Self {
        Self { db, table_name, columns: vec![], rows: vec![], query: String::new() }
    }
}