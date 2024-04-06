use std::{
    env, fs,
    io::{self},
    path::PathBuf,
    process,
};

use chrono::{Local, TimeZone};

use crate::traits::Object;

mod blob;
mod database;
mod entry;
mod traits;
mod tree;
mod workspace;
mod author;
mod commit;

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
            let workspace = workspace::Workspace::new(root_path);
            let database = database::Database::new(db_path);
            let files = workspace.list_files()?;
            let mut entries = Vec::new();
            for file in files {
                let data = workspace.read_data(&file)?;
                let mut blob = blob::Blob::new(&data);
                database.store(&mut blob)?;

                let filename = file.file_name().unwrap().to_str().unwrap().to_string();
                let entry = entry::Entry::new(filename, &blob.object_id);
                //hexdump::hexdump(&blob.object_id.as_bytes());

                entries.push(entry);

            }
            entries.sort_by_key(|e| e.filename.clone());

            let mut tree = tree::Tree::new(entries);
            let _ = database.store(&mut tree).unwrap();

            let name_key = "GIT_AUTHOR_NAME";
            let email_key = "GIT_AUTHOR_EMAIL";
            let name = match env::var(name_key) {
                Ok(name) => name,
                Err(_) => String::from("")
            };
            let email = match env::var(email_key) {
                Ok(email) => email,
                Err(_) => String::from("")
            };
            let now = Local::now();
            let formatted_datetime = now.format("%s %z").to_string();
            let author = author::Author::new(name, email, formatted_datetime);
            
            let mut commit_message = String::new();

            io::stdin().read_line(&mut commit_message)?;
            dbg!(&commit_message);
                
            let mut commit = commit::Commit::new(tree.object_id, author, commit_message.clone());
            let commit_hex_str = database::Database::u8_to_hex_str(commit.object_id.as_bytes().to_vec());
            let _ = database.store(&mut commit).unwrap();

            fs::write(&git_path.join("HEAD"), &commit_hex_str).expect("Unable to write object");
            let first_line = commit.message.lines().next().unwrap();
            println!("[(root-commit) {}] {}", commit_hex_str, first_line);
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
