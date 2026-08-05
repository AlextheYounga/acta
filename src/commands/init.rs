use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use crate::commands::{ACTA_SKILL, CONVENTION_TEMPLATES};
use crate::git::git;
use crate::utils::{add_exclude, write_new};

pub fn init() -> Result<(), String> {
    let home = env::var_os("HOME").ok_or("cannot locate the home directory; `HOME` is not set")?;
    install_skill(&PathBuf::from(home))?;

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

fn install_skill(home: &Path) -> Result<(), String> {
    let skill = home.join(".agents/skills/acta/SKILL.md");
    if skill.exists() {
        return Ok(());
    }
    let directory = skill.parent().ok_or("cannot determine the Acta skill directory")?;
    fs::create_dir_all(directory).map_err(|error| format!("create skill directory: {error}"))?;
    write_new(&skill, ACTA_SKILL)
}

#[cfg(test)]
mod tests {
    use std::error::Error;
    use std::fs;

    use super::{ACTA_SKILL, install_conventions, install_skill};

    #[test]
    fn init_installs_the_acta_skill() -> Result<(), Box<dyn Error>> {
        let home = tempfile::tempdir()?;
        install_skill(home.path())?;

        let skill = home.path().join(".agents/skills/acta/SKILL.md");
        assert!(skill.is_file());
        let installed = fs::read_to_string(&skill)?;
        assert_eq!(installed, ACTA_SKILL);

        fs::write(&skill, "local skill\n")?;
        install_skill(home.path())?;
        assert_eq!(fs::read_to_string(skill)?, "local skill\n");
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
