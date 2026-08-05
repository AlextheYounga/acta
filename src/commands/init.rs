use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use crate::git::git;
use crate::utils::add_exclude;

pub fn init() -> Result<(), String> {
    let home = env::var_os("HOME").ok_or("cannot locate the home directory; `HOME` is not set")?;
    ensure_skill_installed(&PathBuf::from(home))?;

    let root = PathBuf::from(git("rev-parse", &["--show-toplevel"])?);
    fs::create_dir_all(root.join("docs/agents/plans"))
        .map_err(|error| format!("create planning directory: {error}"))?;
    fs::create_dir_all(root.join(".worktrees")).map_err(|error| format!("create worktree directory: {error}"))?;
    add_exclude(&root)?;
    println!("initialized Acta in {}", root.display());
    Ok(())
}

fn ensure_skill_installed(home: &Path) -> Result<(), String> {
    let skill = home.join(".agents/skills/acta/SKILL.md");
    if !skill.is_file() {
        return Err(format!(
            "Acta skill not found at `{}`; install `templates/skills/acta` there before initializing",
            skill.display()
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::error::Error;
    use std::fs;

    use super::ensure_skill_installed;

    #[test]
    fn init_requires_the_acta_skill() -> Result<(), Box<dyn Error>> {
        let home = tempfile::tempdir()?;
        assert!(ensure_skill_installed(home.path()).is_err());

        let skill = home.path().join(".agents/skills/acta");
        fs::create_dir_all(&skill)?;
        fs::write(skill.join("SKILL.md"), "---\nname: acta\n---\n")?;

        assert!(ensure_skill_installed(home.path()).is_ok());
        Ok(())
    }
}
