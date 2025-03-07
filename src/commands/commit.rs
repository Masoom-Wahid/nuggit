use crate::config::CONFIG;
use crate::index::Index;
use crate::commands::NuggitCommand;
use log::debug;
use anyhow::Result;
use crate::utils::files::save_hash_file;
use std::fs::File;
use std::fs;
use crate::commit::Commit;
use chrono::Utc;


pub struct CommitCommand{
    message : String,
    #[allow(dead_code)]
    index : Index
}

impl CommitCommand{
    pub fn new(message: String) -> Result<Self>{
        let index = Index::new(None)?;
        Ok(CommitCommand{message, index})
    }
}


impl NuggitCommand for CommitCommand{
    fn execute(&mut self) -> Result<()> {
        debug!("committing message : {}", self.message);

        let tree = self.index.to_tree()?;
        let tree_hash = tree.hash()?;
        let tree_path = save_hash_file(&tree_hash)?;
        let mut file = File::create(tree_path)?;
        tree.write(&mut file)?;

        let parent = if fs::metadata(CONFIG.head_as_pathbuf()).is_ok() {
            let parent = fs::read_to_string(CONFIG.head_as_pathbuf())?;
            if parent.is_empty(){
                None
            }else{
                Some(parent)
            }
        } else {
            None
        };


        debug!("parent is {:?}",parent);
        let commit = Commit::new(
            tree_hash,
            parent,
            format!("Masoom Wahid <masoom@example.com> {}", Utc::now().to_rfc2822()),
            self.message.clone(),
        );
        
        let commit_hash = commit.hash()?;
        let commit_path = save_hash_file(&commit_hash)?;

        let mut file = File::create(commit_path)?;

        commit.write(&mut file)?;
        
        debug!("commit was compressed and saved");
        
        // debug!("head is {}",CONFIG.head.clone().unwrap());
        fs::write(
            CONFIG.head_as_pathbuf(),
            commit_hash
        )?;
        self.index.reset()?;
        Ok(())
    }
}