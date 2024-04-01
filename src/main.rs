use std::env;
use std::env::current_dir;
use std::fs;
use std::path::PathBuf;
use std::process;
use std::io;

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

fn init(args: &Vec<String>) -> io::Result<()> {
    let default_dir = &"./".to_string();
    let dir: &String = args.get(2).unwrap_or_else(|| default_dir);
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
    let args: Vec<String> = env::args().collect();
    let default_dir = &"./".to_string();
    let cmd: &String = args.get(1).unwrap_or_else(|| default_dir);
    match Command::from(&cmd[..]) {
        Command::Init => {
            match init(&args) {
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
            let root_path = match current_dir() {
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
            let workspace = Workspace::new(root_path.clone());
            let files = workspace.list_files()?;
            for file in files {
                eprintln!("{}", file.as_path().display());
            }
            
        }
        Command::UnknownCommand => {
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
    UnknownCommand
}

impl From<&str> for Command {
    fn from(s: &str) -> Self {
        match s {
            "init" => Command::Init,
            "commit" => Command::Commit,
            _ => Command::UnknownCommand
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
        return Workspace {
            ignore: [".", "..", ".vscode", ".git", "target", "src", ".gitignore"],
            path: path
        };
    }



    fn list_files(&self) -> io::Result<Vec<PathBuf>> {
        let read_files = fs::read_dir(PathBuf::from(&self.path));
        let mut v: Vec<PathBuf> = Vec::new();

        match read_files {
            Ok(files) => {
                for f in files {
                    let dir = f?;
                    let path = dir.path().clone();
                    let mut res = Some(path.clone());
                    for skip in self.ignore {
                        if path.clone().as_path().ends_with(skip) {
                            res = None;
                        }
                    }
                    match res {
                        Some(p) => {
                            v.push(p);
                        }
                        None => {
                            
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
}
    
