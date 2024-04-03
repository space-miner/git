use std::{
  fs,
  io::{self, Read},
  path::PathBuf,
};

use deflate::write::ZlibEncoder;
use deflate::Compression;
use sha1::{Digest, Sha1};
use std::io::Write;
use tempfile::NamedTempFile;
use flate2::read::ZlibDecoder;

use crate::blob;

pub struct Database {
    pub path_buf: PathBuf,
}

impl Database {
    pub fn new(path_buf: PathBuf) -> Self {
        Database { path_buf }
    }

    pub fn store(&self, blob: &mut blob::Blob) -> io::Result<()> {
        let kind = format!("{:?}", blob.kind).to_lowercase();
        let bytesize = blob.data.bytes().len();
        let content_str = format!("{} {}\0{}", kind, bytesize, blob.data);

        let mut hasher = Sha1::new();
        hasher.update(content_str.as_bytes());
        let hash_result = hasher.finalize();
        let content_hash = hash_result.as_slice();
        // hex_output is for output only (display, creating directories/files).
        // the format string expands each hex digit and the resulting string 
        // is not the hash. 
        let mut content_hash_hex = String::new();
        for &byte in content_hash {
            let byte_str = format!("{:X}", byte);
            content_hash_hex.push_str(&byte_str);
        }
        // object_id is the actual hash, we take the hash and interpret it 
        // as a string without any modification of any bits. This requires
        // utf8_unchecked to just read the bits as they are into a string.
        unsafe {
            blob.object_id = String::from_utf8_unchecked(content_hash.to_vec());
        }
        //dbg!(&blob.object_id);
        self.write_object(&content_hash_hex, content_str.as_bytes())?;
        Ok(())
    }

    pub fn write_object(&self, hex_id: &str, content: &[u8]) -> io::Result<()> {
        let hd = &hex_id[0..2];
        let tl = &hex_id[2..];
        dbg!(hex_id);
        let object_path = self.path_buf.join(hd);
        let temp_file = NamedTempFile::new()?;
        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::Fast);
        encoder.write_all(content).expect("Write error!");
        let compressed_data = encoder.finish().expect("Failed to compress object");
        fs::write(&temp_file, compressed_data).expect("Unable to write object");
        fs::create_dir_all(&object_path)?;
        fs::rename(temp_file.path(), object_path.join(tl))?;

        Ok(())
    }

    pub fn inflate(&self, oid: &str) -> String {
        let mut hex_id = String::new();
        let hex_bytes = oid.as_bytes();
        for &byte in hex_bytes {
            let byte_str = format!("{:X}", byte);
            hex_id.push_str(&byte_str);
        }
        let hd = &hex_id[0..2];
        let tl = &hex_id[2..];
        let object_path = self.path_buf.join(hd).join(tl);
        let content = fs::read(object_path.as_path());

        match content {
            Ok(content) => {
                let mut decoder = ZlibDecoder::new(&content[..]);
                let mut s = Vec::new();
                let decompressed = decoder.read_to_end(&mut s);
                match decompressed {
                    Ok(_) => {
                        unsafe {
                            let data = String::from_utf8_unchecked(s);
                            dbg!(data);
                        }

                    },
                    Err(_) => panic!("error decompressing!")
                }
            },
            Err (_) => {
                eprintln!("Could not read object data");
                std::process::exit(1);
            } 
        }

        return hex_id;
    }
}