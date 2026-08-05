pub fn start(topic_type: &str, name: &str) -> Result<(), String> {
    validate_name(topic_type, "topic type")?;
    validate_name(name, "change name")?;

    let root = PathBuf::from(git("rev-parse", &["--show-toplevel"])?);
    let original_branch = git("rev-parse", &["--abbrev-ref", "HEAD"])?;
    if original_branch == "HEAD" {
        return Err("cannot start a plan from a detached HEAD".into());
    }

    let branch = format!("{topic_type}/{name}");
    if git_status("show-ref", &["--verify", &format!("refs/heads/{branch}")])?.status.success() {
        return Err(format!("branch `{branch}` already exists"));
    }
    let worktree = root.join(".worktrees").join(&branch);
    if worktree.exists() {
        return Err(format!("worktree path `{}` already exists", worktree.display()));
    }

    // Git Flow owns topic configuration and branch creation.
    git("flow", &[topic_type, "start", name])?;
    git("checkout", &[&original_branch])
        .map_err(|error| format!("created `{branch}`, but could not restore `{original_branch}`: {error}"))?;

    git("worktree", &["add", &worktree.to_string_lossy(), &branch])?;

    add_exclude(&root)?;
    let plans = worktree.join("docs/agents/plans").join(&branch);
    fs::create_dir_all(&plans).map_err(|error| format!("create plan directory: {error}"))?;
    write_new(&plans.join("01-idea.md"), IDEA_TEMPLATE)?;
    write_new(&plans.join("02-plan.md"), PLAN_TEMPLATE)?;
    write_new(&plans.join("03-tasks.md"), TASKS_TEMPLATE)?;

    println!("branch: {branch}");
    println!("worktree: {}", worktree.display());
    println!("plans: {}", plans.display());
    Ok(())
}

use std::fs;
use std::path::PathBuf;

use crate::commands::{IDEA_TEMPLATE, PLAN_TEMPLATE, TASKS_TEMPLATE};
use crate::git::{git, git_status};
use crate::utils::{add_exclude, validate_name, write_new};
