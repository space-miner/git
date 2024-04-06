use crate::author::Author;
use crate::traits::Object;
use crate::utils;

#[derive(Debug)]
pub struct Commit {
    pub author: Author,
    pub message: String,
    pub object_id: String,
    pub tree_object_id: String,
}

impl Commit {
    pub fn new(tree_object_id: String, author: Author, message: String) -> Self {
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
        let u8 = self.tree_object_id.as_bytes();
        dbg!("here");
        dbg!(utils::u8_to_hex_str(u8.to_vec()));
        let content_str = format!(
            "tree {}\nauthor {}\ncommitter {}\n{}",
            utils::u8_to_hex_str(u8.to_vec()),
            self.author,
            self.author,
            self.message
        );
        format!("commit {}{}", content_str.bytes().len(), content_str)
    }

    fn get_object_id(&self) -> String {
        self.object_id.clone()
    }

    fn set_object_id(&mut self, object_id: String) {
        self.object_id = object_id;
    }
}
