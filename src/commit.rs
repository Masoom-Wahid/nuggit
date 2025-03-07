use serde::{Serialize,Deserialize};
use bincode;
use std::path::PathBuf;
use anyhow::Result;
use std::fs;
use log::{info,debug};
use crate::config::CONFIG;

#[derive(Serialize,Deserialize,Clone,Debug)]
pub struct Commit{
    tree : String,
    parent : Option<String>,
    author : String,
    message : String,
}

impl Commit{
    pub fn new(tree: String, parent: Option<String>, author: String, message: String) -> Self{
        Commit{tree, parent, author, message}
    }

    fn commit_detail(path : &PathBuf) -> Result<Commit> {
        debug!("Reading commit from {}", path.display());
        let first_data = fs::read(path)?;
        let commit_hash = String::from_utf8(first_data)?;
        let data = fs::read(Self::get_path(&commit_hash))?;
        debug!("Data read successfully");
        let commit : Commit = bincode::deserialize(&data)?;
        debug!("Commit deserialized successfully");
        Ok(commit)
    }

    fn get_path(commit_hash : &String) -> PathBuf {
        CONFIG.objects_path.join(commit_hash)
    }
    
    pub fn list_all_commits() -> Result<()> {
        debug!("Listing all commits");
        let current_head = CONFIG.head_as_pathbuf();
        debug!("Current head is {}",current_head.display());
        let mut current_commit = Commit::commit_detail(&current_head)?;
        debug!("Current commit is {:?}",current_commit);
        while current_commit.parent.is_some(){
            debug!("Current commit is {:?}",current_commit);
            let curr_commit_clone = current_commit.clone();
            info!("{}-{}\n{}\n{}",curr_commit_clone.message,curr_commit_clone.author,curr_commit_clone.tree,curr_commit_clone.parent.unwrap());
            let parent_commit = Commit::commit_detail(&PathBuf::from(current_commit.parent.unwrap()))?;
            current_commit = parent_commit;
        }
        Ok(())
    }
}