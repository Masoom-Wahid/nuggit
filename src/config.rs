use std::path::PathBuf;
use lazy_static::lazy_static;
use std::fs;
use anyhow::Result;

pub static GIT_DIR: &str = ".nuggit";

pub struct NuggitConfig{
    pub abs_path : PathBuf,
    pub repo_path : PathBuf,
    pub index_path : PathBuf,
    pub objects_path : PathBuf,
    pub refs_path : PathBuf,
    pub heads_path : PathBuf,
    pub remotes_path : PathBuf,
    pub head : Option<String>,
    pub current_parent : Option<String>,
}

impl NuggitConfig{
    fn get_current_parent(head_path : &Result<String>) -> Result<String>{
        if let Ok(head_path) = head_path {
            let head_content = fs::read_to_string(head_path)?;
            Ok(head_content)
        } else {
            Ok("".to_string())
        }
    }
    fn parse_head(heads_path : &PathBuf) -> Result<String>{
        let head_path = heads_path.clone();
        let head_content = fs::read_to_string(head_path)?;
        let head_content = head_content.split(":").collect::<Vec<&str>>();
        let head_name = head_content[1].trim();
        Ok(head_name.to_string())
    }

    pub fn head_as_pathbuf(&self) -> PathBuf{
        return self.repo_path.join(self.head.clone().unwrap());
    }

    pub fn new() -> Self{
        let abs_path = PathBuf::from(".");
        let repo_path = abs_path.join(GIT_DIR);
        let index_path = repo_path.join("index");
        let objects_path = repo_path.join("objects");
        let refs_path = repo_path.join("refs");
        let heads_path = repo_path.join("HEAD");
        let remotes_path = repo_path.join("remotes");
        let head = Self::parse_head(&heads_path);
        let current_parent = Self::get_current_parent(&head);
        Self{
            abs_path,
            repo_path,
            index_path,
            objects_path,
            refs_path,
            heads_path,
            remotes_path,
            head: head.ok(),
            current_parent : current_parent.ok()
        }
    }
}



lazy_static!{
    pub static ref CONFIG: NuggitConfig = NuggitConfig::new();
}