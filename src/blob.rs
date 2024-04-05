use crate::traits::Object;
use std::fmt;

#[derive(fmt::Debug)]
pub enum Kind {
    Blob,
    Tree,
}

#[derive(Debug)]
pub struct Blob {
    pub data: String,
    pub kind: Kind,
    pub object_id: String,
}

impl Blob {
    pub fn new(data: &str) -> Self {
        Blob {
            data: data.to_string(),
            kind: Kind::Blob,
            object_id: String::from(""),
        }
    }

    
}

impl Object for Blob {
    fn to_string(&self) -> String {
        let kind = format!("{:?}", self.kind).to_lowercase();
        let bytesize = self.data.bytes().len();
        return format!("{} {}\0{}", kind, bytesize, self.data);
    }

    fn get_object_id(&self) -> String {
        return self.object_id.clone();
    }

    fn set_object_id(&mut self, object_id: String) {
        self.object_id = object_id;
    }
  }
