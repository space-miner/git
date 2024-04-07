use std::{error, fmt, fs::{self, File, OpenOptions}, io::{self, Write}, path::PathBuf};

#[derive(Debug)]
pub enum LockfileError {
    MissingParent,
    NoPermission,
    StaleLock,
    UnknownError
}

impl error::Error for LockfileError {}

impl fmt::Display for LockfileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            err => write!(f, "{}", err)
        }
    }
}

pub struct LockFile {
    file_path: PathBuf,
    lock_path: PathBuf,
    lock: Option<File>
}

impl LockFile {
    pub fn new(path: PathBuf) -> Self {
        Self {
            file_path: path.join("HEAD"),
            lock_path: path.join("HEAD.lock").clone(),
            lock: None
        }
    }

    

    pub fn hold_for_update(&mut self) -> Result<bool, LockfileError> {
        match &self.lock {
            Some(_) => {
                Ok(true)
            },
            None => {
                match OpenOptions::new()
                    .read(true)
                    .write(true)
                    .create(true)
                    .open(&self.lock_path) {
                    Ok(lock) => {    
                        self.lock = Some(lock);
                        Ok(true)
                    },
                    Err(ref err) if err.kind() == io::ErrorKind::AlreadyExists => {
                        Ok(false)
                    },
                    Err(ref err) if err.kind() == io::ErrorKind::NotFound => {
                        dbg!(err,&self.lock_path);
                        Err(LockfileError::MissingParent)
                    }
                    Err(ref err) if err.kind() == io::ErrorKind::PermissionDenied => {
                        Err(LockfileError::NoPermission)
                    }
                    Err (_) => {
                        panic!("error in hold_for_update");
                    }
                }
            }
        }
    }

    pub fn write(&self, s: String) -> Result<(), LockfileError> {
        match self.raise_on_stale_lock() {
            Ok(_) => {
                let mut lock = self.lock.as_ref().unwrap();
                let _ = lock.write_all(s.as_bytes());
                //unchecked result
                Ok(())
            },
            Err(err) => {
                Err(err)
            }
        }
    }
 
    pub fn commit(&mut self) -> Result<(), LockfileError> {
        self.raise_on_stale_lock()?;
        dbg!(&self.lock_path, &self.file_path);
        dbg!(&self.lock_path, &self.file_path);
        let result = fs::rename(&self.lock_path, &self.file_path);
        match result {
            Ok(_) => {
                self.lock = None;
                Ok(())
            },
            Err(_) => {
                panic!("Error in lockfile commit");
            }
        }


    }
    
    pub fn raise_on_stale_lock(&self) -> Result<(), LockfileError> {
        match &self.lock {
            Some(_) => Ok(()),
            None => Err(LockfileError::StaleLock)
        }
    }
    
    }


    
