use std::{fs::{self, File}, io::{self, Read}, path::{Path, PathBuf}};

#[derive(Debug)]
pub struct Refs {
    pub pathname: PathBuf
}

impl Refs {
    pub fn new(pathname: PathBuf) -> Self {
        Self {
            pathname
        }
    }

    pub fn update_head(&self, commit_hex_str: String) {
        fs::write(self.head_path(), &commit_hex_str).expect("Unable to write commit to HEAD file.");
    }

    pub fn head_path (&self) -> PathBuf {
        self.pathname.join("HEAD")
    }

    pub fn read_head (&self) -> io::Result<String> {
        let head_path = self.head_path();
        let path = head_path.as_path();
        if path.exists() {
            let mut file = File::open(&path)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            Ok(contents)
        } else {
            Ok(String::from(""))
        }
    }
}