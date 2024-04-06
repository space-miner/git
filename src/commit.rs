use crate::author;
use crate::traits::Object;

#[derive(Debug)]
pub struct Commit {
    pub author: author::Author,
    pub message: String,
    pub object_id: String,
    pub tree_object_id: String,
}

impl Commit {
    pub fn new(tree_object_id: String, author: author::Author, message: String) -> Self {
        Commit {
            author,
            message,
            tree_object_id,
            object_id: String::from(""),
        }
    }
}

impl Object for Commit {
    fn to_string(&self) -> String {
        format!(
            "tree {}\nauthor {}\ncommitter {}\n{}",
            self.tree_object_id, self.author, self.author, self.message
        )
    }

    fn get_object_id(&self) -> String {
        self.object_id.clone()
    }

    fn set_object_id(&mut self, object_id: String) {
        self.object_id = object_id;
    }
}
