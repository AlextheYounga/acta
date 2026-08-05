use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use crate::commands::CONVENTION_TEMPLATES;
use crate::git::git;
use crate::utils::{add_exclude, write_new};

pub fn init() -> Result<(), String> {
    let home = env::var_os("HOME").ok_or("cannot locate the home directory; `HOME` is not set")?;
    ensure_skill_installed(&PathBuf::from(home))?;

    let root = PathBuf::from(git("rev-parse", &["--show-toplevel"])?);
    fs::create_dir_all(root.join("docs/agents/plans"))
        .map_err(|error| format!("create planning directory: {error}"))?;
    install_conventions(&root)?;
    fs::create_dir_all(root.join(".worktrees")).map_err(|error| format!("create worktree directory: {error}"))?;
    add_exclude(&root)?;
    println!("initialized Acta in {}", root.display());
    Ok(())
}

fn install_conventions(root: &Path) -> Result<(), String> {
    let directory = root.join("docs/agents/conventions");
    fs::create_dir_all(&directory).map_err(|error| format!("create conventions directory: {error}"))?;
    for (name, contents) in CONVENTION_TEMPLATES {
        let path = directory.join(name);
        if !path.exists() {
            write_new(&path, contents)?;
        }
    }
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

    use super::{ensure_skill_installed, install_conventions};

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

    #[test]
    fn init_installs_missing_conventions_without_overwriting_them() -> Result<(), Box<dyn Error>> {
        let root = tempfile::tempdir()?;
        install_conventions(root.path())?;

        let convention = root.path().join("docs/agents/conventions/clean-code.md");
        assert!(convention.is_file());
        fs::write(&convention, "local convention\n")?;

        install_conventions(root.path())?;
        assert_eq!(fs::read_to_string(convention)?, "local convention\n");
        Ok(())
    }
}
