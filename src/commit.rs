use serde::{Serialize,Deserialize};
use bincode;
use std::path::PathBuf;
use anyhow::Result;
use std::fs;
use std::fs::File;
use log::{info,debug};
use crate::config::CONFIG;
use crate::utils::files::{compress,decompress};
use std::io::Write;
use chrono::Local;
use crate::utils::files::hash;


#[derive(Serialize,Deserialize,Clone,Debug)]
pub struct CommitTree{
    pub nodes : Vec<CommitTreeNode>
}

impl CommitTree{

    pub fn write(&self, file: &mut File) -> Result<()> {
        let serialized = bincode::serialize(self)?;
        let compressed = compress(&serialized)?;
        file.write_all(&compressed)?;
        Ok(())
    }

    pub fn hash(&self) -> Result<String> {
        let serialized = bincode::serialize(self)?;
        let hash = hash(&serialized)?;
        Ok(hash)
    }

    pub fn read(path: &PathBuf) -> Result<Self> {
        let compressed = std::fs::read(path)?;
        let decompressed = decompress(&compressed)?;
        let tree = bincode::deserialize(&decompressed)?;
        Ok(tree)
    }
}


#[derive(Serialize,Deserialize,Clone,Debug)]
pub struct CommitTreeNode{
    pub mode: String,
    pub name: String,
    pub hash: String,
}

impl CommitTreeNode{
    pub fn new(mode: String, name: String, hash: String) -> Self{
        CommitTreeNode{mode, name, hash}
    }
}

#[derive(Serialize,Deserialize,Clone,Debug)]
pub struct Commit{
    tree : String,
    parent : Option<String>,
    author : String,
    message : String,
    timestamp : String
}

impl Commit{
    pub fn new(tree: String, parent: Option<String>, author: String, message: String) -> Self{
        let timestamp = Local::now().to_string();
        Commit{tree, parent, author, message, timestamp}
    }

    pub fn write(&self,file : &mut File) -> Result<()> {
        file.write_all(
            compress(
                bincode::serialize(&self)?.as_slice()
            )?.as_slice()
        )?;
        Ok(())
    }

    pub fn hash(&self) -> Result<String> {
        let serialized = bincode::serialize(self)?;
        let hash = hash(&serialized)?;
        Ok(hash)
    }

    pub fn read(path : &PathBuf) -> Result<Commit> {
        let data = fs::read(path)?;
        let commit : Commit = bincode::deserialize(&decompress(&data)?)?;
        Ok(commit)
    }

    fn get_path(commit_hash : &String) -> PathBuf {
        let dir_path = CONFIG.objects_path.join(commit_hash[0..2].to_string());
        let file_path = dir_path.join(commit_hash[2..].to_string());
        file_path
    }
    
    fn print_commit(commit : Option<Commit>){
        if let Some(commit) = commit {
            info!(
                "\nCommit: {}\nAuthor: {}\nParent: {}\n",
                commit.message,
                commit.author,
                commit.parent.unwrap_or("None".to_string())
            );
        }
    }
    pub fn list_all_commits() -> Result<()> {
        debug!("Listing all commits");
        let head_ref = fs::read_to_string(CONFIG.head_as_pathbuf())?;
        
        debug!("Current head is {}",head_ref);

        let mut current_commit = Some(
            Commit::read(
                &Self::get_path(&head_ref)
            )?
        );
        Self::print_commit(current_commit.clone());
        debug!("Current commit is {:?}",current_commit);
        while current_commit.is_some() {
            if let Some(commit) = &current_commit {
                if let Some(parent) = &commit.parent {
                    let parent_commit = Commit::read(
                        &Self::get_path(parent)
                    )?;
                    current_commit = Some(parent_commit);
                    Self::print_commit(current_commit.clone());
                } else {
                    break;
                }
            }
        }
        Ok(())
    }
}