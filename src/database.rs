use std::{fs, io, path::PathBuf};

use deflate::write::ZlibEncoder;
use deflate::Compression;
use std::io::Write;
use tempfile::NamedTempFile;

use crate::traits::Object;
use crate::utils;

pub struct Database {
    pub path_buf: PathBuf,
}

impl Database {
    pub fn new(path_buf: PathBuf) -> Self {
        Database { path_buf }
    }

    pub fn store(&self, object: &mut dyn Object) -> io::Result<()> {
        let content_str = object.to_string();
        let content_hash = utils::hash_content(&content_str);
        let content_hash_hex = utils::u8_to_hex_str(content_hash.clone());
        unsafe {
            object.set_object_id(String::from_utf8_unchecked(content_hash));
        }
        self.write_object(&content_hash_hex, content_str.as_bytes())?;
        Ok(())
    }

    pub fn write_object(&self, content_hash_hex: &str, content: &[u8]) -> io::Result<()> {
        let (dir, file) = utils::hash_to_path(content_hash_hex);
        let object_path = self.path_buf.join(dir);
        if fs::metadata(object_path.join(file)).is_ok() {
            return Ok(())
        }
        let temp_file = NamedTempFile::new()?;
        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::Fast);
        encoder.write_all(content).expect("Write error!");
        let compressed_data = encoder.finish().expect("Failed to compress object");
        fs::write(&temp_file, compressed_data).expect("Unable to write object");
        fs::create_dir_all(&object_path)?;
        fs::rename(temp_file.path(), object_path.join(file))?;

        Ok(())
    }
}
