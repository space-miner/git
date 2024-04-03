#[derive(Debug)]
pub enum BlobKind {
    Blob,
}

#[derive(Debug)]
pub struct Blob {
    pub data: String,
    pub kind: BlobKind,
    pub object_id: String,
}

impl Blob {
    pub fn new(data: &str) -> Self {
        Blob {
            data: data.to_string(),
            kind: BlobKind::Blob,
            object_id: String::from("")
        }
    }
}