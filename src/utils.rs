use flate2::read::ZlibDecoder;
use sha1::{Digest, Sha1};
use std::{env, fs, io::Read, path::PathBuf, process};

pub fn inflate(object_path: PathBuf) -> String {
    match fs::read(object_path.as_path()) {
        Ok(content) => {
            let mut decoder = ZlibDecoder::new(&content[..]);
            let mut s = Vec::new();
            let decompressed = decoder.read_to_end(&mut s);
            match decompressed {
                Ok(_) => unsafe { String::from_utf8_unchecked(s) },
                Err(_) => panic!("error decompressing!"),
            }
        }
        Err(e) => {
            eprintln!("Could not read object data");
            std::process::exit(1);
        }
    }
}

pub fn hash_content(content_str: &str) -> Vec<u8> {
    let mut hasher = Sha1::new();
    hasher.update(content_str.as_bytes());
    let hash_result = hasher.finalize();
    hash_result.as_slice().to_vec()
}

pub fn u8_to_hex_str(content_hash: Vec<u8>) -> String {
    let mut content_hash_hex = String::new();
    for byte in &content_hash {
        let byte_str = format!("{:02X}", byte);
        content_hash_hex.push_str(&byte_str);
    }
    content_hash_hex
}

pub fn hash_to_path(content_hash_hex: &str) -> (&str, &str) {
    (&content_hash_hex[0..2], &content_hash_hex[2..])
}

pub fn get_root_path() -> PathBuf {
    match env::current_dir() {
        Ok(cwd) => cwd,
        Err(_) => {
            eprintln!("current_dir() failure in commit case.");
            process::exit(1);
        }
    }
}

pub fn get_git_path() -> PathBuf {
    let root_path = get_root_path();
    let mut git_path = PathBuf::from(&root_path);
    git_path.push(".git");
    git_path
}

pub fn get_db_path() -> PathBuf {
    let git_path = get_git_path();
    let mut db_path = PathBuf::from(&git_path);
    db_path.push("objects");
    db_path
}
