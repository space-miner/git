use std::{error, fmt, fs::{self, File}, io::{self, Read}, path::{Path, PathBuf}};

use crate::lockfile;

#[derive(Debug)]
pub enum RefsError {
    LockDenied
}

impl error::Error for RefsError {}

impl fmt::Display for RefsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            err => write!(f, "{}", err)
        }
    }
}

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

    pub fn update_head(&self, commit_hex_str: String) -> Result<(), RefsError> {
        let mut lockfile = lockfile::LockFile::new(self.head_path());
        match lockfile.hold_for_update() {
            Ok(true) => {
                // uncaught results!
                let _ = lockfile.write(commit_hex_str);
                let _ = lockfile.write(String::from("\n"));
                let _ = lockfile.commit();
                Ok(())
            },
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

    pub fn create_head_dir (&self) -> io::Result<()> {
        dbg!("here");
        fs::create_dir(self.head_path())?;
        Ok(())
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
