use std::path::PathBuf;

#[derive(Debug)]
pub struct Entry {
    pub path: PathBuf,
    pub object_id: String,
}

impl Entry {
    pub fn new(path: PathBuf, object_id: &str) -> Self {
        Entry {
            path,
            object_id: object_id.to_string(),
        }
    }
}
