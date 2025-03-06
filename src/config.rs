use std::path::PathBuf;
use lazy_static::lazy_static;

pub static GIT_DIR: &str = ".nuggit";

pub struct NuggitConfig{
    pub abs_path : PathBuf,
    pub repo_path : PathBuf,
    pub index_path : PathBuf,
    pub objects_path : PathBuf,
    pub refs_path : PathBuf,
    pub heads_path : PathBuf,
    pub remotes_path : PathBuf,
}

impl NuggitConfig{
    pub fn new() -> Self{
        let abs_path = PathBuf::from(".");
        let repo_path = abs_path.join(GIT_DIR);
        let index_path = repo_path.join("index");
        let objects_path = repo_path.join("objects");
        let refs_path = repo_path.join("refs");
        let heads_path = repo_path.join("HEAD");
        let remotes_path = repo_path.join("remotes");
        Self{abs_path, repo_path, index_path, objects_path, refs_path, heads_path, remotes_path}
    }
}



lazy_static!{
    pub static ref CONFIG: NuggitConfig = NuggitConfig::new();
}