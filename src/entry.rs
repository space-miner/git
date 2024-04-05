#[derive(Debug)]
pub struct Entry {
    pub filename: String,
    pub object_id: String,
}

impl Entry {
    pub fn new(filename: String, object_id: &str) -> Self {
        Entry {
            filename,
            object_id: object_id.to_string(),
        }
    }
}
