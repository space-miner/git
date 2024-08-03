use std::collections::HashMap;
use std::path::PathBuf;

use crate::blob::Kind;
use crate::database::Database;
use crate::entry::Entry;
use crate::traits::Object;

#[derive(Debug)]
enum EntryOrTree {
    Entry(Entry),
    Tree(Tree),
}

#[derive(Debug)]
pub struct Tree {
    pub entries_order: Vec<String>,
    pub entries: HashMap<String, EntryOrTree>,
    pub kind: Kind,
    pub object_id: String,
}

impl Tree {
    pub fn new() -> Self {
        Self {
            entries_order: Vec::new(),
            entries: HashMap::new(),
            kind: Kind::Tree,
            object_id: String::new(),
        }
    }

    pub fn build(mut entries: Vec<Entry>) -> Self {
        entries.sort_by_key(|e| e.filename.clone());
        let mut root = Self::new();
        for entry in entries {
            let ancestors = entry.ancestor_directories();
            root.add_entry(ancestors, entry)
        }
        root
    }

    pub fn store_tree(&mut self, db: &Database) {
        for (_, entry_or_tree) in &mut self.entries {
            if let EntryOrTree::Tree(subtree) = entry_or_tree {
                subtree.store_tree(&db)
            }
        }
        Database::store(&db, self);
    }

    pub fn add_entry(&mut self, parents: Vec<PathBuf>, entry: Entry) {
        if parents.is_empty() {
            self.entries_order.push(entry.filename.clone());
            self.entries
                .insert(entry.filename.clone(), EntryOrTree::Entry(entry));
        } else {
            if !self.entries.contains_key(&entry.filename) {
                self.entries
                    .insert(entry.filename.clone(), EntryOrTree::Tree(Tree::new()));
            }
            let parents = &parents[1..];
            self.add_entry(parents.to_vec(), entry)
        }
    }

    // fn mode(&self) -> String {
    //     return String::from("40000");
    // }
}

impl Object for Tree {
    /*
       Tree format:
       /* TODO: update comment */
       -> update this <file mode> <file name>\0<object id>

       Note that object id is the hash of the tree object, so
       it is not displayable.
    */
    fn to_string(&self) -> String {
        let kind = format!("{:?}", self.kind).to_lowercase();
        let mut content = String::new();

        for (filename, entry_or_tree) in &self.entries {
            let (mode, object_id) = match entry_or_tree {
                EntryOrTree::Entry(entry) => (entry.mode(), entry.object_id.clone()),
                EntryOrTree::Tree(tree) => (String::from("40000"), tree.object_id.clone()),
            };

            content.push_str(&format!("{} {}\0{}", mode, filename, object_id,))
        }
        // metadata + content
        format!("{} {}\0{}", kind, content.bytes().len(), content)
    }

    fn get_object_id(&self) -> String {
        self.object_id.clone()
    }

    fn set_object_id(&mut self, object_id: String) {
        self.object_id = object_id;
    }
}
