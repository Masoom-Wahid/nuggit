use crate::commands::NuggitCommand;
use anyhow::Result;
use std::path::Path;
use std::fs::read_dir;
use crate::utils::files::{hash,compress};
use log::{debug,info};
use crate::config::CONFIG;
use fs_extra::dir::create_all;
use std::fs::File;
use std::io::Write;
use crate::index::Index;
use serde::{Serialize,Deserialize};
use bincode;
pub struct AddCommand{
    pub path : String,
    index : Index
}

#[derive(Serialize,Deserialize)]
struct ObjectContent{
    file_name : String,
    hash_str : String,
    content : Vec<u8>
}

impl AddCommand{
    pub fn new(path : String) -> Result<Self>{
        debug!("creating index");
        let index = Index::new(None)?;
        debug!("finished creating index");
        Ok(Self{
            path,
            index
        })

    }
    fn object_content(&self, file_name: &str, hash_str: &str, content: &[u8]) -> Result<Vec<u8>> {
        let object_content = ObjectContent{
            file_name : file_name.to_string(),
            hash_str : hash_str.to_string(),
            content : content.to_vec()
        };
        let serialized = bincode::serialize(&object_content)?;
        Ok(serialized)
    }

    fn exec_add_dir(&mut self, path : &Path) -> Result<()>{
        let path = Path::new(&path);

        for entry in read_dir(path)?{
            let entry = entry?;
            let entry_path = entry.path();
            if entry_path.is_dir(){
                self.exec_add_dir(&entry_path)?;
            } else {
                self.exec_add(&entry_path)?;
            }
        }
        Ok(())
    }

    fn exec_add(&mut self, path : &Path) -> Result<String>{
        let content = std::fs::read(path)?;
        let hash_str = hash(&content)?;

        debug!("Hash of {} is {}", path.display(), hash_str);

        let object_dir = hash_str[0..2].to_string();
        let object_dir_path = CONFIG.objects_path.join(object_dir);
        let object_path = object_dir_path.join(hash_str[2..].to_string());
        if !object_path.exists() {
            create_all(&object_dir_path,false)?;
            debug!("file was created at {}",object_path.display());
        }

        let object_path = object_dir_path.join(hash_str[2..].to_string());
        let mut file = File::create(object_path)?;

        // debug!("file was created at {:?}",object_path.display());

        let object_content = self.object_content(
            path.file_name().unwrap().to_str().unwrap(),
            &hash_str,
            &content
        )?;         
        debug!("created content");

        file.write_all(
            compress(&object_content)?.as_slice()
        )?;
        debug!("file was compressed and saved");

        self.index.add_entry(path.to_str().unwrap(), &hash_str)?;
        debug!("added entry to index");
        Ok(hash_str)
    }
}

impl NuggitCommand for AddCommand{
    
    fn execute(&mut self) -> Result<()> {
        let add_path = self.path.clone();
        let path = Path::new(&add_path);
        if !path.exists() {
            return Err(anyhow::anyhow!("File does not exist: {}", self.path));
        }
        if path.is_dir(){
            self.exec_add_dir(&path)?;
        }else{
            self.exec_add(path)?;
        }
        self.index.write()?;
        info!("added {}",self.path);
        Ok(())
    }
}