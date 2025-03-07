use crate::commands::NuggitCommand;
use anyhow::Result;
use log::{info,warn,debug};
use fs_extra::dir::create_all;
use fs_extra::file::write_all;
use std::path::Path;
use std::fs::remove_dir_all;
use crate::config::{CONFIG,GIT_DIR};
pub struct InitCommand{
    #[allow(dead_code)]
    // TODO: use this path to initiialize 
    // and also since lazy static is used
    // i cant really change the abs path right now
    pub path: Option<String>,
}

impl InitCommand{
    pub fn new(path: Option<String>) -> Self{
        InitCommand{path}
    }
}
impl NuggitCommand for InitCommand {
    fn execute(&mut self) -> Result<()> {
        let should_delete = true;
        info!("Initializing project in path : {:?}", CONFIG.abs_path);

        let git_path = Path::new(&CONFIG.repo_path);
        if git_path.exists() && git_path.is_dir(){
            warn!("{} already exists", GIT_DIR);
            if should_delete {
                remove_dir_all(git_path)?;
                info!("Deleted existing directory");
            } else {
                return Err(anyhow::anyhow!("{} already exists", GIT_DIR));
            }
        }

        create_all(git_path, false)?;
        debug!("Created git directory {}", GIT_DIR);
        
        /*
        For some reason if the index file is created here,
        i get end of file when running add command
         */
        // File::create(Path::new(&CONFIG.index_path))?;
        // debug!("Created {} index file", CONFIG.index_path.display());
        
        write_all(
            Path::new(&CONFIG.heads_path),
            "ref: refs/heads/main\n"
        )?;
        debug!("Created HEAD file pointing to main branch");

        create_all(
            Path::new(&CONFIG.objects_path),
            false
        )?;
        debug!("Created objects directory {}", CONFIG.objects_path.display());

        create_all(
            Path::new(&CONFIG.refs_path.join("heads")),
            false
        )?;
        debug!("Created refs directory {}", CONFIG.refs_path.display());
        

        debug!("Creating the main branch head {}", CONFIG.refs_path.join("heads/main").display());
        write_all(
            Path::new(&CONFIG.refs_path.join("heads/main")),
            ""
        )?;
        debug!("Created default branch ref at {}", CONFIG.refs_path.join("heads/main").display());

        
        let config_content = "[core]\n\trepositoryformatversion = 0\n";
        write_all(
            Path::new(&CONFIG.repo_path.join("config")),
            config_content
        )?;
        debug!("Created config file at {}", CONFIG.repo_path.join("config").display());
        
        create_all(Path::new(&CONFIG.remotes_path), false)?;
        debug!("Created remotes directory {}", CONFIG.remotes_path.display());

        let readme_path = CONFIG.repo_path.join("README");
        write_all(
            Path::new(&readme_path),
            "Welcome to Nuggit -  A worse version of git !\n"
        )?;
        debug!("Added a little Nuggit readme");
        Ok(())
    }
}