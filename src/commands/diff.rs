use crate::config::CONFIG;
use crate::commands::NuggitCommand;
use crate::commit::{Commit, CommitTree};
use crate::utils::files::hash;
use anyhow::Result;
use log::{debug, info};
use std::fs;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use crate::commands::add::ObjectContent;

pub struct DiffCommand {}

impl DiffCommand {
    pub fn new() -> Self {
        DiffCommand {}
    }


    fn build_working_tree() -> Result<HashMap<String, String>> {
        let mut working_tree = HashMap::new();
        for entry in fs::read_dir(".")? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && path.file_name().unwrap() != ".nuggit" {
                let name = path.file_name().unwrap().to_string_lossy().to_string();
                let content = fs::read_to_string(&path)?;
                let content_hash = hash(content.as_bytes())?;
                working_tree.insert(name, content_hash);
            }
        }
        Ok(working_tree)
    }


    fn compute_diff(old_content: &str, new_content: &str) -> String {
        let old_lines: Vec<&str> = old_content.lines().collect();
        let new_lines: Vec<&str> = new_content.lines().collect();
        let mut diff = String::new();

        let max_len = old_lines.len().max(new_lines.len());
        for i in 0..max_len {
            match (old_lines.get(i), new_lines.get(i)) {
                (Some(old), Some(new)) if old == new => continue,
                (Some(old), Some(new)) => {
                    diff.push_str(&format!("- {}\n", old));
                    diff.push_str(&format!("+ {}\n", new));
                }
                (Some(old), None) => diff.push_str(&format!("- {}\n", old)),
                (None, Some(new)) => diff.push_str(&format!("+ {}\n", new)),
                (None, None) => break,
            }
        }
        diff
    }

    fn get_path(hash: &str) -> PathBuf {
        let prefix = &hash[0..2];
        let suffix = &hash[2..];
        CONFIG.objects_path.join(prefix).join(suffix)
    }
}

impl NuggitCommand for DiffCommand {
    fn execute(&mut self) -> Result<()> {
        debug!("Running diff command");

        
        let head_hash = fs::read_to_string(CONFIG.head_as_pathbuf())?;
        debug!("head_hash: {}", head_hash);
        let commit = Commit::read(&Self::get_path(&head_hash))?;
        debug!("commit: {:?}", commit);
        let tree = CommitTree::read(&Self::get_path(&commit.tree))?;
        debug!("tree: {:?}", tree);
        let mut committed_tree: HashMap<String, String> = HashMap::new();
        for node in &tree.nodes {
            committed_tree.insert(node.name.clone(), node.hash.clone());
        }
        debug!("committed_tree: {:?}", committed_tree);
        let working_tree = Self::build_working_tree()?;
        debug!("working_tree: {:?}", working_tree);
        let mut changes = Vec::new();

        for (name, committed_hash) in &committed_tree {
            match working_tree.get(name) {
                Some(working_hash) if working_hash != committed_hash => {
                    debug!("modified file: {}", name);
                    debug!("committed_hash: {}", committed_hash);
                    let old_content_obj = ObjectContent::read(&Self::get_path(committed_hash))?;
                    let old_content = String::from_utf8(old_content_obj.content)?;
                    debug!("old_content: {:?}", old_content);
                    let new_content = fs::read_to_string(Path::new(name))?;
                    debug!("new_content: {}", new_content);
                    let diff = Self::compute_diff(&old_content, &new_content);
                    debug!("diff: {}", diff);
                    changes.push(format!("diff {}\n{}", name, diff));
                }
                None => {
                    changes.push(format!("deleted {}", name));
                }
                _ => {}
            }
        }

        for (name, _) in &working_tree {
            if !committed_tree.contains_key(name) {
                changes.push(format!("added {}", name));
            }
        }

        if changes.is_empty() {
            info!("No changes found");
        } else {
            for change in changes {
                info!("{}", change);
            }
        }

        Ok(())
    }
}