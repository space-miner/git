#[derive(Debug)]
pub struct Author {
    pub name: String,
    pub email: String,
    pub time: String
}

impl Author {
    pub fn new(name: String, email: String, time: String) -> Self {
        Author {
            name,
            email,
            time
        }
    }

    pub fn to_string(&self) -> String {
        format!("{} {} {}", self.name, self.email, self.time)
  }
}