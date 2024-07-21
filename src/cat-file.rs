use std::env;
use std::io;
use std::io::Write;
use std::path::Path;

mod utils;

fn main() -> io::Result<()> {
    let db_path = utils::get_db_path();
    let args: Vec<String> = env::args().collect();
    let path_str = args.get(1).expect("expect path to object file");
    let path_buf = Path::new(path_str);
    let object_filename = path_buf.file_name().unwrap();
    let object_dir = path_buf.parent().unwrap().file_name().unwrap();
    let object_path = db_path.join(object_dir).join(object_filename);
    let object_contents = utils::inflate(object_path);
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    handle.write_all(object_contents.as_bytes())?;

    Ok(())
}
