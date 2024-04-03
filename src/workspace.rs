use std::{
  fs,
  io::{self},
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
                      v.push(path.clone());
                  };
              }
          }
          Err(_) => {
              eprintln!("error reading files in current directory");
              process::exit(1);
          }
      }
      Ok(v)
  }
}