use std::fs;
use std::path::PathBuf;

use crate::commands::CLARIFICATION_TEMPLATE;
use crate::git::git;
use crate::utils::{validate_name, write_new};

pub fn clarify(name: &str) -> Result<(), String> {
    validate_name(name, "clarification name")?;
    let branch = git("rev-parse", &["--abbrev-ref", "HEAD"])?;
    if branch == "HEAD" {
        return Err("cannot clarify a detached HEAD".into());
    }
    let plans = plan_directory(&branch)?;
    if !plans.is_dir() {
        return Err(format!("no Acta plan directory at `{}`", plans.display()));
    }

    let next = fs::read_dir(&plans)
        .map_err(|error| format!("read plan directory: {error}"))?
        .filter_map(Result::ok)
        .filter_map(|entry| entry.file_name().to_str().map(str::to_owned))
        .filter_map(|file| file.get(0..2)?.parse::<u32>().ok())
        .max()
        .unwrap_or(3)
        + 1;
    let path = plans.join(format!("{next:02}-{name}-clarity.md"));
    write_new(&path, CLARIFICATION_TEMPLATE)?;
    println!("{}", path.display());
    Ok(())
}

fn plan_directory(branch: &str) -> Result<PathBuf, String> {
    if branch == "HEAD" || branch.is_empty() {
        return Err("the current checkout has no branch".into());
    }
    let root = PathBuf::from(git("rev-parse", &["--show-toplevel"])?);
    Ok(root.join("docs/agents/plans").join(branch))
}
