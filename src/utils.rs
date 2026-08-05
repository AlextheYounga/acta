use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::git::git;

pub fn write_new(path: &Path, contents: &str) -> Result<(), String> {
    OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .and_then(|mut file| file.write_all(contents.as_bytes()))
        .map_err(|error| format!("write `{}`: {error}", path.display()))
}

pub fn add_exclude(root: &Path) -> Result<(), String> {
    let common = PathBuf::from(git("rev-parse", &["--git-common-dir"])?);
    let common = if common.is_absolute() { common } else { root.join(common) };
    let info = common.join("info");
    fs::create_dir_all(&info).map_err(|error| format!("create Git info directory: {error}"))?;
    let exclude = info.join("exclude");
    let existing = fs::read_to_string(&exclude).unwrap_or_default();
    if existing.contains("# begin acta") {
        return Ok(());
    }
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&exclude)
        .map_err(|error| format!("open Git exclude file: {error}"))?;
    if !existing.is_empty() && !existing.ends_with('\n') {
        writeln!(file).map_err(|error| error.to_string())?;
    }
    writeln!(file, "# begin acta\n/.worktrees/\n# end acta").map_err(|error| format!("write Git exclude file: {error}"))
}

pub fn validate_name(value: &str, label: &str) -> Result<(), String> {
    if value.is_empty()
        || value == "."
        || value == ".."
        || value.starts_with('-')
        || value
            .chars()
            .any(|character| character.is_whitespace() || !matches!(character, 'a'..='z' | '0'..='9' | '-' | '_' | '.'))
    {
        return Err(format!("invalid {label} `{value}`; use lowercase letters, numbers, `-`, `_`, or `.`"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::validate_name;

    #[test]
    fn names_reject_path_traversal_and_shell_punctuation() {
        assert!(validate_name("../outside", "name").is_err());
        assert!(validate_name("change name", "name").is_err());
        assert!(validate_name("safe-change", "name").is_ok());
    }
}
