use core::panic;
use std::default;
use std::env;
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

fn main() {
    let args: Vec<String> = env::args().collect();
    let default_dir = &"./".to_string();
    let cmd: &String = args.get(1).unwrap_or_else(|| default_dir);
    match Command::from(&cmd[..]) {
        Command::Init => {
            match init(&args) {
                Ok(_) => {
                    println!("init success")
                }
                Err(_) => {
                    eprintln!("init failure");
                    std::process::exit(1);
                }
            }
        }
        Command::Commit => {
            let workspace = Workspace::new("./");
            todo!()
        }
        Command::UnknownCommand => {
            eprintln!("Usage: {} <command> [<directory>]", args[0]);
            process::exit(1);
        }
    }
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
    ignore: [&'static str; 2],
    path_name: String,
}

impl Workspace {
    fn new(path_name: &str) -> Self {
        return Workspace {
            ignore: [".", ".."],
            path_name: path_name.to_string(),
        };
    }
}
