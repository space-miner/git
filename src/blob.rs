#[derive(Debug)]
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
