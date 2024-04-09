use std::{
    fs::{self, Metadata},
    io,
    path::{Path, PathBuf},
    process,
};

#[derive(Debug)]
pub struct Workspace {
    ignore: [&'static str; 7],
    path: PathBuf,
}

impl Workspace {
    pub fn new(path: PathBuf) -> Self {
        Workspace {
            ignore: [".", "..", ".vscode", ".git", "target", "src", ".gitignore"],
            path,
        }
    }

    pub fn read_data(&self, path: &Path) -> io::Result<String> {
        fs::read_to_string(path)
    }

    pub fn list_files(&self) -> io::Result<Vec<PathBuf>> {
        let read_files_res = fs::read_dir(PathBuf::from(&self.path));
        let mut v = Vec::new();

        match read_files_res {
            Ok(read_files) => {
                for file in read_files {
                    let path = file?.path();
                    if self.ignore.into_iter().all(|x| !path.ends_with(x)) {
                        if path.is_dir() {
                            let dir = Self::new(path.clone());
                            let mut files_from_dir = dir.list_files()?;
                            v.append(&mut files_from_dir);
                        } else if path.is_file() {
                            v.push(path.clone());
                        }
                    }
                }
            }
            Err(_) => {
                eprintln!("error reading files in current directory");
                process::exit(1);
            }
        }
        Ok(v)
    }

    pub fn stat_file(&self, path: PathBuf) -> Metadata {
        match fs::metadata(path) {
            Ok(metadata) => metadata,
            Err(_) => {
                panic!("Could not stat_file in workspace");
            }
        }
    }
}
