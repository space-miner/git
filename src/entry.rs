use std::{
    fs::Metadata,
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone)]
pub struct Entry {
    pub filename: String,
    pub path: PathBuf,
    pub object_id: String,
    pub stat: Metadata,
}

impl Entry {
    pub fn new(filename: String, path: PathBuf, object_id: &str, stat: Metadata) -> Self {
        Entry {
            filename,
            path,
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

    pub fn ancestor_directories(&self) -> Vec<PathBuf> {
        let ancestors = Path::new(self.path.as_path())
            .ancestors()
            .map(PathBuf::from)
            .collect::<Vec<PathBuf>>();
        let slice = &ancestors[1..ancestors.len() - 1];
        let mut rev = slice.to_vec();
        rev.reverse();
        rev
    }
}
