use crate::tree;
use crate::author;
use crate::traits::Object;


#[derive(Debug)]
pub struct Commit {
    pub author: author::Author,
    pub message: String,
    pub object_id: String
}

impl Commit {
    pub fn new(object_id: String, author: author::Author, message: String) -> Self {
        Commit {
            author,
            message,
            object_id
        }
    }

    
}

impl Object for Commit {

    fn to_string(&self) -> String {

        format!("tree {}\nauthor {}\ncommitter {}\n{}", 
                self.object_id,
                self.author.to_string(),
                self.author.to_string(),
                self.message)
    }

    fn get_object_id(&self) -> String {
        self.object_id.clone()
    }

    fn set_object_id(&mut self, object_id: String) {
        self.object_id = object_id;
    }
}