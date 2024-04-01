use std::{
    env, fs, io,
    path::{Path, PathBuf},
    process,
};

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
        dbg!("creating new directory {:?}", dir);
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
            let files = workspace.list_files()?;
            for file in files {
                let data = workspace.read_data(&file)?;
                let blob = Blob::new(&data);
                println!("data:{}, blob: {:?}", data, blob);
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
    //object_id:
}

impl Blob {
    fn new(data: &str) -> Self {
        Blob {
            data: data.into(),
            kind: BlobKind::Blob, // object_id:
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
}
