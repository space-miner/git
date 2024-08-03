use std::{
    error, fmt,
    fs::File,
    io::{self, Read},
    path::PathBuf,
};

use crate::lockfile;

#[derive(Debug)]
pub enum RefsError {
    LockDenied,
}

impl error::Error for RefsError {}

impl fmt::Display for RefsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let err = self;
        write!(f, "{}", err)
    }
}

#[derive(Debug)]
pub struct Refs {
    pub pathname: PathBuf,
}

impl Refs {
    pub fn new(pathname: PathBuf) -> Self {
        Self { pathname }
    }

    pub fn update_head(&self, commit_hex_str: String) -> Result<(), RefsError> {
        let mut lockfile = lockfile::LockFile::new(self.pathname.clone());
        match lockfile.hold_for_update() {
            Ok(true) => {
                // uncaught results!
                let _ = lockfile.write(commit_hex_str);
                let _ = lockfile.write(String::from("\n"));
                let _ = lockfile.commit();
                Ok(())
            }
            Ok(false) => {
                dbg!("false in update head");
                Err(RefsError::LockDenied)
            }
            Err(err) => {
                dbg!(err);
                Err(RefsError::LockDenied)
            }
        }
    }

    pub fn head_path(&self) -> PathBuf {
        self.pathname.join("HEAD")
    }

    pub fn read_head(&self) -> io::Result<String> {
        let head_path = self.head_path();
        let path = head_path.as_path();
        if path.exists() {
            let mut file = File::open(path)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            let contents = contents.trim_end_matches('\n').to_string();
            Ok(contents)
        } else {
            Ok(String::from(""))
        }
    }
}
