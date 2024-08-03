use std::collections::HashMap;
use std::path::PathBuf;

use crate::blob::Kind;
use crate::database::Database;
use crate::entry::Entry;
use crate::traits::Object;

#[derive(Debug)]
pub enum EntryOrTree {
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

    pub fn build(entries: Vec<Entry>) -> Self {
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
        let _ = Database::store(&db, self);
    }

    pub fn add_entry(&mut self, parents: Vec<PathBuf>, entry: Entry) {
        if parents.is_empty() {
            self.entries_order.push(entry.filename.clone());
            self.entries
                .insert(entry.filename.clone(), EntryOrTree::Entry(entry));
        } else {
            dbg!(&parents); 
            let path = &parents[0];
            // foo/bar/world.txt   bar/world.txt   

            let first_component = path.components().next().unwrap();
            let mut basename = String::new();
            match first_component {
                std::path::Component::RootDir => {
                    eprintln!("The path starts with a root directory.");
                    panic!();
                }
                std::path::Component::Normal(component) => {
                    basename = String::from(component.to_str().unwrap());
                }
                _ => {
                    println!("The first component is not a directory.");
                    panic!();
                }
            }

            let entry_or_tree = self.entries.entry(basename.clone()).or_insert(EntryOrTree::Tree(Tree::new()));
            self.entries_order.push(basename.clone());
            let parents = &parents[1..];
            dbg!(&parents);
            // TODO: this sucks what's the best way of not having to wrap unwrap this bogus layer
            match entry_or_tree {
                EntryOrTree::Tree(subtree) => subtree.add_entry(parents.to_vec(), entry),
                _ => eprint!("not suppose to be here"),
            }
        }
    }
}

impl Object for Tree {

    fn to_string(&self) -> String {
        let kind = format!("{:?}", self.kind).to_lowercase();
        let mut content = String::new();

        for filename in &self.entries_order {
            let entry_or_tree = self.entries.get(filename).unwrap();
            let (mode, object_id) = match entry_or_tree {
                EntryOrTree::Entry(entry) => (entry.mode(), entry.object_id.clone()),
                EntryOrTree::Tree(tree) => (String::from("40000"), tree.object_id.clone()),
            };

            content.push_str(&format!("{} {}\0{}", mode, filename, object_id,))
        }
        format!("{} {}\0{}", kind, content.bytes().len(), content)
    }

    fn get_object_id(&self) -> String {
        self.object_id.clone()
    }

    fn set_object_id(&mut self, object_id: String) {
        self.object_id = object_id;
    }
}
