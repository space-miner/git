use crate::blob::Kind;
use crate::entry::Entry;

#[derive(Debug)]
pub struct Tree {
    pub entries: Vec<Entry>,
    pub mode: String,
    pub kind: Kind,
    pub object_id: String,
}

impl Tree {
    pub fn new(entries: Vec<Entry>) -> Self {
        Self {
            entries,
            mode: "100644".to_string(),
            kind: Kind::Tree,
            object_id: String::new(),
        }
    }

    pub fn to_string(&self) -> String {
        let kind = format!("{:?}", self.kind).to_lowercase();
        let mut content = String::new();
        for entry in &self.entries {
            content.push_str(&format!("{} {}\0{}", self.mode, entry.filename, entry.object_id))
        }
        // metadata + content
        return format!("{} {}\0{}", kind, content.bytes().len(), content);
    }
}