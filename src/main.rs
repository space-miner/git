use std::env;
use std::fs;
use std::path;
use std::path::PathBuf;
use std::process;

fn init(mut path_buf: PathBuf) {
    path_buf.push(".git");
    let dirs = ["objects", "refs"];
    for dir in dirs.into_iter() {
        path_buf.push(dir);
        let _ = fs::create_dir_all(&path_buf);
        path_buf.pop();
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <command> [<directory>]", args[0]);
        process::exit(1);
    }
    let cmd = args[1].as_str();
    match cmd {
        "init" => {
            let cwd = "./".to_string();
            let dir = args.get(2).unwrap_or(&cwd);
            let path_buf_result = fs::canonicalize(dir);
            match path_buf_result {
                Ok(path_buf) => {
                    init(path_buf);
                }
                Err(_) => {
                    let path = path::Path::new(dir);
                    let _ = fs::create_dir_all(&path);
                    let path_buf = path.into();
                    init(path_buf);
                }
            }
        }
        _ => todo!("unimplemented command {}", cmd),
    }
}
