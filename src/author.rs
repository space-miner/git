use std::fmt;

#[derive(Debug)]
pub struct Author {
    pub name: String,
    pub email: String,
    pub time: String,
}

impl Author {
    pub fn new(name: String, email: String, time: String) -> Self {
        Author { name, email, time }
    }
}

impl fmt::Display for Author {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.name, self.email, self.time)
    }
}
