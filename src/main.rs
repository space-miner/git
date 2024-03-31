use std::env;
use std::fs;
use std::path::PathBuf;
use std::process;

fn initialize_repo_directory(mut path_buf: PathBuf) -> std::io::Result<()> {
    path_buf.push(".git");
    let dirs = ["objects", "refs"];
    for dir in dirs.into_iter() {
        path_buf.push(dir);
        fs::create_dir_all(&path_buf)?;
        path_buf.pop();
    }
    Ok(())
}

fn init(args: Vec<String>) -> std::io::Result<()> {
    let dir = args.get(2).unwrap_or_else(|| &"./".to_string());
    let path = fs::canonicalize(*dir).or_else(|_| {
        fs::create_dir_all(dir)?;
        dbg!("creating new directory {:?}", dir);
        Ok(PathBuf::from(dir))
    })?;
    println!("Initialized empty Git repository in {}", path.display());
    initialize_repo_directory(path)?;
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let cmd = args.get(1).unwrap_or_else(|| &"./".to_string());
    match Command::from(*cmd) {
        Command::Init => {
            todo!()
        }
        Command::Commit => {
            let workspace = Workspace::new("./");
            todo!()
        }
        _ => {
            eprintln!("Usage: {} <command> [<directory>]", args[0]);
            process::exit(1);
        }
    }
}

#[derive(Debug)]
enum Command {
    Init,
    Commit,
}

impl From<&str> for Command {
    fn from(s: &str) -> Self {
        match s {
            "init" => Command::Init,
            "commit" => Command::Commit,
        }
    }
}

impl From<String> for Command {
    fn from(s: String) -> Self {
        match &s[..] {
            "init" => Command::Init,
            "commit" => Command::Commit,
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
