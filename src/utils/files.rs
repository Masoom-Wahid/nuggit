use sha1::{Sha1, Digest};
#[allow(unused_imports)]
use flate2::{Compression, bufread::ZlibEncoder};
use std::io::Write;
use crate::config::CONFIG;
use fs_extra::dir::create_all;
use log::debug;
use anyhow::Result;

pub fn hash(data : &[u8]) -> Result<String>{
    let mut hasher = Sha1::new();
    hasher.update(data);
    let hash = hasher.finalize();
    let hash_str = format!("{:x}", hash);
    Ok(hash_str)
}


pub fn compress(data : &[u8]) -> Result<Vec<u8>>{
    let mut compressor = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::default());
    compressor.write_all(data)?;
    let compressed = compressor.finish()?;
    Ok(compressed)
}

pub fn save_hash_file(hash : &str) -> Result<String>{
    let hash_dir = hash[0..2].to_string();
    let hash_dir_path = CONFIG.objects_path.join(hash_dir);
    let hash_path = hash_dir_path.join(hash[2..].to_string());
    if !hash_path.exists() {
        create_all(&hash_dir_path,false)?;
        debug!("file was created at {}",hash_path.display());
    }

    let hash_path = hash_dir_path.join(hash[2..].to_string());
    Ok(hash_path.to_string_lossy().to_string())
}