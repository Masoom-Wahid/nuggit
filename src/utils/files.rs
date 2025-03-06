use sha1::{Sha1, Digest};
#[allow(unused_imports)]
use flate2::{Compression, bufread::ZlibEncoder};
use anyhow::Result;
use std::io::Write;

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