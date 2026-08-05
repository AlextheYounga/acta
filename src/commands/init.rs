use std::fs;
use std::path::PathBuf;

use crate::git::git;
use crate::utils::add_exclude;

pub fn init() -> Result<(), String> {
    let root = PathBuf::from(git("rev-parse", &["--show-toplevel"])?);
    fs::create_dir_all(root.join("docs/agents/plans"))
        .map_err(|error| format!("create planning directory: {error}"))?;
    fs::create_dir_all(root.join(".worktrees"))
        .map_err(|error| format!("create worktree directory: {error}"))?;
    add_exclude(&root)?;
    println!("initialized Acta in {}", root.display());
    Ok(())
}
