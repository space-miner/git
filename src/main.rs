use std::{
    env, fs, io::{self, Read},
    path::{Path, PathBuf},
    process,
};

use hex_literal::hex;
use sha1::{Sha1, Digest};
use tempfile::NamedTempFile;
use std::fs::OpenOptions;
use std::io::Write;
use deflate::Compression;
use deflate::write::ZlibEncoder;


fn initialize_repo_directory(mut path_buf: PathBuf) -> io::Result<()> {
    path_buf.push(".git");
    let dirs = ["objects", "refs"];
    for dir in dirs.into_iter() {
        path_buf.push(dir);
        fs::create_dir_all(&path_buf)?;
        path_buf.pop();
    }
    Ok(())
}

fn init(dir: &str) -> io::Result<()> {
    let path: PathBuf = fs::canonicalize(dir).or_else(|_| {
        fs::create_dir_all(dir)?;
        Ok::<PathBuf, io::Error>(PathBuf::from(dir))
    })?;
    println!("Initialized empty Git repository in {}", path.display());
    initialize_repo_directory(path)?;
    Ok(())
}

fn main() -> io::Result<()> {
    let args = env::args().collect::<Vec<String>>();
    let cmd = args.get(1).expect("Usage: {} <command> [<directory>]");
    match Command::from(&cmd[..]) {
        Command::Init => {
            let default_dir = &"./".to_string();
            let dir = args.get(2).unwrap_or(default_dir);
            match init(dir) {
                Ok(_) => {
                    println!("init success");
                }
                Err(_) => {
                    eprintln!("init failure");
                    std::process::exit(1);
                }
            }
        }
        Command::Commit => {
            let root_path = match env::current_dir() {
                Ok(cwd) => cwd,
                Err(_) => {
                    eprintln!("current_dir() failure in commit case.");
                    process::exit(1);
                }
            };
            let mut git_path = PathBuf::from(&root_path);
            git_path.push(".git");
            let mut db_path = PathBuf::from(&git_path);
            db_path.push("objects");
            println!("git path:{}", git_path.display());
            println!("db path: {}", db_path.display());
            let workspace = Workspace::new(root_path);
            let database = Database::new(db_path);
            let files = workspace.list_files()?;
            for file in files {
                let data = workspace.read_data(&file)?;
                let mut blob = Blob::new(&data);
                database.store(& mut blob)?;
            }
    
        }
        Command::Unknown => {
            eprintln!("Usage: {} <command> [<directory>]", args[0]);
            process::exit(1);
        }
    }
    Ok(())
}

#[derive(Debug)]
enum Command {
    Init,
    Commit,
    Unknown,
}

impl From<&str> for Command {
    fn from(s: &str) -> Self {
        match s {
            "init" => Command::Init,
            "commit" => Command::Commit,
            _ => Command::Unknown,
        }
    }
}

#[derive(Debug)]
struct Workspace {
    ignore: [&'static str; 7],
    path: PathBuf,
}

impl Workspace {
    fn new(path: PathBuf) -> Self {
        Workspace {
            ignore: [".", "..", ".vscode", ".git", "target", "src", ".gitignore"],
            path,
        }
    }

    fn read_data(&self, p: &Path) -> io::Result<String> {
        fs::read_to_string(p)
    }

    fn list_files(&self) -> io::Result<Vec<PathBuf>> {
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

#[derive(Debug)]
enum BlobKind {
    Blob,
}

#[derive(Debug)]
struct Blob {
    data: String,
    kind: BlobKind,
    object_id: String
}

impl Blob {
    fn new(data: &str) -> Self {
        Blob {
            data: data.into(),
            kind: BlobKind::Blob, 
            object_id: String::from("")
        }
    }
}

struct Database {
    path: PathBuf,
}

impl Database {
    fn new(pbuf: PathBuf) -> Self {
        Database { path: pbuf }
    }

    fn store(&self, blob: & mut Blob) -> io::Result<()> {
      let mut hasher = Sha1::new();
      hasher.update(blob.data.as_bytes());
      let result = hasher.finalize();
      let u8slice = result.as_slice();
      let mut s = String::new();
      for &byte in u8slice {
          let byte_str = String::from(format!("{:X}", byte));
          s.push_str(&byte_str);
      }
      blob.object_id = s;

      let kind = format!("{:?}", blob.kind).to_lowercase();
      let bytesize = blob.data.len();
      let content = format!("{} {}\0{}", kind, bytesize, blob.data);
      self.write_object(&blob.object_id, &content)?;
      Ok(())
    }

    fn write_object(&self, oid: &String, content: &String) -> io::Result<()> {
      let hd = &oid[0..2];
      let tl = &oid[2..];
      let mut object_path = self.path.join(hd);
      let file = NamedTempFile::new()?;
      let mut encoder = ZlibEncoder::new(Vec::new(), Compression::Default);
      encoder.write_all(content.as_bytes()).expect("Write error!");
      let compressed_data = encoder.finish().expect("Failed to compress object");
      unsafe {
          let compressed_string = String::from_utf8_unchecked(compressed_data);
          fs::write(&file, compressed_string).expect("Unable to write object");
      }
      fs::create_dir_all(&object_path)?;
      dbg!(&object_path);
      fs::rename(&file.path(), &object_path.join(tl))?;

      Ok(())

      


//       object_path = @pathname.join(oid[0..1], oid[2..-1])
// dirname
// = object_path.dirname
// temp_path
// = dirname.join(generate_temp_name)
      
    }

    

}
