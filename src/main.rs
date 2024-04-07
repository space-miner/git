use std::{env, fs, io, path::PathBuf, process};

use chrono::Local;

mod author;
mod blob;
mod commit;
mod database;
mod entry;
mod traits;
mod tree;
mod utils;
mod workspace;

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
            // set up paths.
            let git_path = utils::get_git_path();
            let db_path = utils::get_db_path();
            let root_path = utils::get_root_path();

            // set up git data structures.
            let workspace = workspace::Workspace::new(root_path);
            let database = database::Database::new(db_path);

            // Read current workspace files into Entry vector.
            let files = workspace.list_files()?;
            let mut entries = Vec::new();
            for file in files {
                let data = workspace.read_data(&file)?;
                let mut blob = blob::Blob::new(&data);
                database.store(&mut blob)?;
                let filename = file.file_name().unwrap().to_str().unwrap().to_string();
                let entry = entry::Entry::new(filename, &blob.object_id);
                entries.push(entry);
            }
            entries.sort_by_key(|e| e.filename.clone());

            // Create and store tree for commit. 
            let mut tree = tree::Tree::new(entries);
            database.store(&mut tree).unwrap();

            // Create Author. 
            let now = Local::now();
            let formatted_datetime = now.format("%s %z").to_string();
            let author_name = env::var("GIT_AUTHOR_NAME").expect("GIT_AUTHOR_NAME not set");
            let author_email = env::var("GIT_AUTHOR_EMAIL").expect("GIT_AUTHOR_EMAIL not set");
            let author = author::Author::new(author_name, author_email, formatted_datetime);

            // Read commit message, create commit, store it. 
            let mut commit_message = String::new();
            io::stdin().read_line(&mut commit_message)?;
            let mut commit = commit::Commit::new(tree.object_id, author, commit_message.clone());
            database.store(&mut commit).unwrap();
            
            // Write commit id to HEAD.
            let commit_hex_str = utils::u8_to_hex_str(commit.object_id.as_bytes().to_vec());
            fs::write(git_path.join("HEAD"), &commit_hex_str).expect("Unable to write object");
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
