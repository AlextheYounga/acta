use std::io::Write;
use std::iter::once;
use std::process::{Command, Stdio};

use dialoguer::Select;

use crate::git::git;

#[derive(Debug, PartialEq, Eq)]
struct Worktree {
    path: String,
    branch: String,
}

pub fn worktrees() -> Result<(), String> {
    let output = git("worktree", &["list", "--porcelain"])?;
    let entries = parse_worktrees(&output)?;
    let choices: Vec<String> = entries.iter().map(|entry| format!("{}  [{}]", entry.path, entry.branch)).collect();
    let selection = Select::new()
        .with_prompt("Choose a worktree")
        .items(&choices)
        .interact()
        .map_err(|error| format!("choose a worktree: {error}"))?;
    let path = &entries[selection].path;
    copy_to_clipboard(path)?;
    println!("copied {path}");
    Ok(())
}

fn parse_worktrees(output: &str) -> Result<Vec<Worktree>, String> {
    let mut entries = Vec::new();
    let mut path = None;
    let mut branch = None;

    for line in output.lines().chain(once("")) {
        if let Some(value) = line.strip_prefix("worktree ") {
            path = Some(value.to_owned());
        } else if let Some(value) = line.strip_prefix("branch ") {
            branch = Some(value.strip_prefix("refs/heads/").unwrap_or(value).to_owned());
        } else if line.is_empty() {
            if let Some(path) = path.take() {
                entries.push(Worktree { path, branch: branch.take().unwrap_or_else(|| "detached HEAD".to_owned()) });
            }
        }
    }

    if entries.is_empty() {
        return Err("no worktrees found in the current repository".into());
    }
    Ok(entries)
}

fn copy_to_clipboard(path: &str) -> Result<(), String> {
    let backends = [
        ("wl-copy", Vec::new()),
        ("pbcopy", Vec::new()),
        ("xclip", vec!["-selection", "clipboard"]),
        ("xsel", vec!["--clipboard", "--input"]),
    ];
    let mut failures = Vec::new();

    for (program, args) in backends {
        let Ok(mut child) = Command::new(program).args(args).stdin(Stdio::piped()).spawn() else {
            continue;
        };
        let Some(mut stdin) = child.stdin.take() else {
            failures.push(format!("`{program}` did not accept input"));
            continue;
        };
        if let Err(error) = stdin.write_all(path.as_bytes()) {
            failures.push(format!("write to `{program}`: {error}"));
            continue;
        }
        drop(stdin);
        match child.wait() {
            Ok(status) if status.success() => return Ok(()),
            Ok(status) => failures.push(format!("`{program}` exited with {status}")),
            Err(error) => failures.push(format!("wait for `{program}`: {error}")),
        }
    }

    if failures.is_empty() {
        Err("could not copy path: install `wl-copy`, `pbcopy`, `xclip`, or `xsel`".into())
    } else {
        Err(format!("could not copy path; install `wl-copy`, `pbcopy`, `xclip`, or `xsel`: {}", failures.join("; ")))
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::{Worktree, parse_worktrees};

    #[test]
    fn parses_worktree_paths_and_branches() -> Result<(), Box<dyn Error>> {
        let output = "worktree /repo\nHEAD abc\nbranch refs/heads/main\n\nworktree /repo/.worktrees/feat/demo\nHEAD def\nbranch refs/heads/feat/demo\n\n";

        assert_eq!(
            parse_worktrees(output)?,
            vec![
                Worktree { path: "/repo".into(), branch: "main".into() },
                Worktree { path: "/repo/.worktrees/feat/demo".into(), branch: "feat/demo".into() },
            ]
        );
        Ok(())
    }

    #[test]
    fn labels_detached_worktrees() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            parse_worktrees("worktree /repo\nHEAD abc\n detached\n")?,
            vec![Worktree { path: "/repo".into(), branch: "detached HEAD".into() }]
        );
        Ok(())
    }
}
