use crate::blob::Kind;
use crate::entry::Entry;
use crate::traits::Object;

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
}
impl Object for Tree {
    fn to_string(&self) -> String {
        let kind = format!("{:?}", self.kind).to_lowercase();
        let mut content = String::new();
        for entry in &self.entries {
            content.push_str(&format!("{} {}\0{}", self.mode, entry.filename, entry.object_id))
        }
        // metadata + content
        return format!("{} {}\0{}", kind, content.bytes().len(), content);
    }

    fn get_object_id(&self) -> String {
        return self.object_id.clone();
    }
    
    fn set_object_id(&mut self, object_id: String) {
        self.object_id = object_id;
    }
}