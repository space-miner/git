use std::{fs::Metadata, os::unix::fs::PermissionsExt};

#[derive(Debug)]
pub struct Entry {
    pub filename: String,
    pub object_id: String,
    pub stat: Metadata,
}

impl Entry {
    pub fn new(filename: String, object_id: &str, stat: Metadata) -> Self {
        Entry {
            filename,
            object_id: object_id.to_string(),
            stat,
        }
    }

    pub fn mode(&self) -> String {
        let permissions = self.stat.permissions();
        // Check if owner has executable permission set.
        if permissions.mode() & 0o100 != 0 {
            String::from("100755")
        } else {
            String::from("100644")
        }
    }
}
