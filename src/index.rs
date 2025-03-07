use std::path::PathBuf;
use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, BufWriter, Read, Write};
use anyhow::Result;
use serde::{Serialize, Deserialize};
use bincode;
use crate::config::CONFIG;
use log::debug;
use std::collections::HashMap;
use crate::commit::{CommitTree,CommitTreeNode};


#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct Entry {
    path: PathBuf,
    hash: String,
    mode: u32,
    flags: u32,
}


pub struct Index {
    path: PathBuf,
    entries: HashMap<String,Entry>,
}

impl Index {
    fn fill_entries(&mut self) -> Result<()> {
        debug!("filling entries");
        debug!("opening file {}", self.path.display());
        let file = match File::open(&self.path) {
            Ok(f) => f,
            Err(e) if e.kind() == io::ErrorKind::NotFound => {
                self.entries = HashMap::new();
                return Ok(());
            }
            Err(e) => return Err(e.into()),
        };

        let mut reader = BufReader::new(file);
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;

    
        let entries: Vec<Entry> = bincode::deserialize(&buffer)?;

        self.entries = entries.into_iter()
            .map(|entry| (entry.path.to_string_lossy().into_owned(), entry))
            .collect();

        Ok(())
    }

    pub fn add_entry(&mut self, path: &str, hash: &str) -> Result<()> {
        let entry = Entry {
            path: PathBuf::from(path),
            hash: String::from(hash),
            mode: 0,
            flags: 0,
        };
        if self.entries.contains_key(path) {
            self.entries.get_mut(path).unwrap().hash = hash.to_string();
        }else{
            self.entries.insert(path.to_string(), entry);
        }
        Ok(())
    }

    pub fn to_tree(&self) -> Result<CommitTree> {
        let mut nodes = Vec::new();
        for (path, entry) in &self.entries {
            let path_buf = PathBuf::from(path);
            let name = path_buf
                .file_name()
                .ok_or_else(|| anyhow::anyhow!("Invalid path: {:?}", path))?
                .to_string_lossy()
                .to_string();

            let node = CommitTreeNode{
                mode: entry.mode.to_string(),
                name,
                hash: entry.hash.clone(),
            };
            nodes.push(node);
        }
        Ok(CommitTree { nodes })
    }

    pub fn write(&self) -> Result<()> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.path)?;

        let mut writer = BufWriter::new(file);
        let entries = self.entries.values().cloned().collect::<Vec<_>>();
        let encoded = bincode::serialize(&entries)?;
        writer.write_all(&encoded)?;
        writer.flush()?;
        Ok(())
    }

    pub fn list_entries(&self) -> Result<()> {
        for entry in self.entries.values(){
            println!("{} - {}", entry.path.display(), entry.hash);
        }
        Ok(())
    }

    pub fn new(path: Option<PathBuf>) -> Result<Self> {
        let path = path.unwrap_or(CONFIG.index_path.clone());
        let mut index = Index {
            path,
            entries: HashMap::new()
        };
        index.fill_entries()?;
        Ok(index)
    }

    pub fn reset(&mut self) -> Result<()> {
        self.entries = HashMap::new();
        self.write()?;
        Ok(())
    }
}