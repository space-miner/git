use std::{
    fs,
    io::{self, Read},
    path::PathBuf,
};

use deflate::write::ZlibEncoder;
use deflate::Compression;
use flate2::read::ZlibDecoder;
use sha1::{Digest, Sha1};
use std::io::Write;
use tempfile::NamedTempFile;

use crate::traits::Object;

pub struct Database {
    pub path_buf: PathBuf,
}

impl Database {
    pub fn new(path_buf: PathBuf) -> Self {
        Database { path_buf }
    }

    pub fn store(&self, object: &mut dyn Object) -> io::Result<()> {
        let content_str = object.to_string();
        let content_hash = Self::hash_content(&content_str);
        let content_hash_hex = Self::u8_to_hex_str(content_hash.clone());
        unsafe {

            object.set_object_id(String::from_utf8_unchecked(content_hash));
        }
        self.write_object(&content_hash_hex, content_str.as_bytes())?;
        Ok(())
    }

    pub fn write_object(&self, content_hash_hex: &str, content: &[u8]) -> io::Result<()> {
        let (dir, file) = Self::hash_to_path(content_hash_hex);
        let object_path = self.path_buf.join(dir);
        let temp_file = NamedTempFile::new()?;
        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::Fast);
        encoder.write_all(content).expect("Write error!");
        let compressed_data = encoder.finish().expect("Failed to compress object");
        fs::write(&temp_file, compressed_data).expect("Unable to write object");
        fs::create_dir_all(&object_path)?;
        fs::rename(temp_file.path(), object_path.join(file))?;

        Ok(())
    }

    

    pub fn inflate(&self, object_id: &str) -> String {
        let content_hash = object_id.as_bytes().to_vec();
        let content_hash_hex = Self::u8_to_hex_str(content_hash.clone());
        let (dir, file) = Self::hash_to_path(&content_hash_hex);
        
        let object_path = self.path_buf.join(dir).join(file);
        let compressed_content = fs::read(object_path.as_path());

        match compressed_content {
            Ok(content) => {
                let mut decoder = ZlibDecoder::new(&content[..]);
                let mut s = Vec::new();
                let decompressed = decoder.read_to_end(&mut s);
                match decompressed {
                    Ok(_) => unsafe {
                        let data = String::from_utf8_unchecked(s);
                        data
                    },
                    Err(_) => panic!("error decompressing!"),
                }
            }
            Err(_) => {
                eprintln!("Could not read object data");
                std::process::exit(1);
            }
        }
    }

    pub fn hash_content(content_str: &str) -> Vec<u8> {
        let mut hasher = Sha1::new();
        hasher.update(content_str.as_bytes());
        let hash_result = hasher.finalize();
        return hash_result.as_slice().to_vec();
    }

    pub fn u8_to_hex_str(content_hash: Vec<u8>) -> String {
        let mut content_hash_hex = String::new();
        for byte in &content_hash {
            let byte_str = format!("{:02X}", byte);
            content_hash_hex.push_str(&byte_str);
        }
        return content_hash_hex;
    }

    pub fn hash_to_path(content_hash_hex: &str) -> (&str, &str) {
        return(&content_hash_hex[0..2], &content_hash_hex[2..]);
    }

}
