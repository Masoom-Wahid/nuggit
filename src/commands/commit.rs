use crate::config::CONFIG;
use crate::{index::Index, utils::files::compress};
use crate::commands::NuggitCommand;
use log::debug;
use anyhow::Result;
use crate::utils::files::{hash,save_hash_file};
use bincode;
use std::fs::File;
use std::io::Write;
use std::fs;
use crate::commit::Commit;


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
        let tree_hash = hash(tree.as_bytes())?;
        let commit = Commit::new(
                    tree, 
                    CONFIG.current_parent.clone(),
            "Masoom Wahid".to_string(),
            self.message.clone()
        );
        let tree_path = save_hash_file(&tree_hash)?;
        let mut file = File::create(tree_path)?;
        file.write_all(
            compress(
                bincode::serialize(&commit)?.as_slice()
            )?.as_slice()
        )?;
        debug!("tree was compressed and saved");
        debug!("head is {}",CONFIG.head.clone().unwrap());
        fs::write(
            //TODO: maybae not use unwrap everytime ???
            CONFIG.head_as_pathbuf(),
            tree_hash
        )?;
        self.index.reset()?;
        // debug!("Head is updated to {}",tree_hash.to_string());
        Ok(())
    }
}