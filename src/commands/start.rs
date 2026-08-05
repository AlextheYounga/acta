pub fn start(topic_type: &str, name: &str) -> Result<(), String> {
    validate_name(topic_type, "topic type")?;
    validate_name(name, "change name")?;

    let (flow_command, prefix) = resolve_topic_type(topic_type)?;
    let branch = format!("{prefix}{name}");

    let root = PathBuf::from(git("rev-parse", &["--show-toplevel"])?);
    let original_branch = git("rev-parse", &["--abbrev-ref", "HEAD"])?;
    if original_branch == "HEAD" {
        return Err("cannot start a plan from a detached HEAD".into());
    }

    if git_status("show-ref", &["--verify", &format!("refs/heads/{branch}")])?.status.success() {
        return Err(format!("branch `{branch}` already exists"));
    }
    let worktree = root.join(".worktrees").join(&branch);
    if worktree.exists() {
        return Err(format!("worktree path `{}` already exists", worktree.display()));
    }

    // Git Flow owns topic configuration and branch creation.
    git("flow", &[&flow_command, "start", name])?;
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

fn resolve_topic_type(selector: &str) -> Result<(String, String), String> {
    let configured = git("config", &["--get-regexp", r"^gitflow\.branch\..+\.type$"])?;

    for line in configured.lines() {
        let Some((key, kind)) = line.split_once(char::is_whitespace) else {
            continue;
        };
        if kind.trim() != "topic" {
            continue;
        }

        let Some(command) = key.strip_prefix("gitflow.branch.").and_then(|key| key.strip_suffix(".type")) else {
            continue;
        };
        let prefix_key = format!("gitflow.branch.{command}.prefix");
        let prefix = git("config", &["--get", &prefix_key])?;
        if selector == command || prefix.trim_end_matches('/') == selector {
            return Ok((command.to_owned(), prefix));
        }
    }

    Err(format!("no Git Flow topic type matches `{selector}`; use a configured topic command or prefix"))
}

use std::fs;
use std::path::PathBuf;

use crate::commands::{IDEA_TEMPLATE, PLAN_TEMPLATE, TASKS_TEMPLATE};
use crate::git::{git, git_status};
use crate::utils::{add_exclude, validate_name, write_new};
