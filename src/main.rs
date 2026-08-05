use clap::{Parser, Subcommand};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

const IDEA_TEMPLATE: &str = include_str!("../templates/planning/01-idea.md");
const PLAN_TEMPLATE: &str = include_str!("../templates/planning/02-plan.md");
const TASKS_TEMPLATE: &str = include_str!("../templates/planning/03-tasks.md");
const CLARIFICATION_TEMPLATE: &str = include_str!("../templates/planning/clarity.md");

#[derive(Parser)]
#[command(name = "acta", about = "Create and maintain lightweight change plans")]
struct Cli {
    #[command(subcommand)]
    command: CommandKind,
}

#[derive(Subcommand)]
enum CommandKind {
    Init,
    Start { branch: String },
    Clarify { name: String },
}

fn main() {
    if let Err(error) = run(Cli::parse()) {
        eprintln!("acta: {error}");
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> Result<(), String> {
    match cli.command {
        CommandKind::Init => init(),
        CommandKind::Start { branch } => {
            let (topic_type, name) = parse_topic_branch(&branch)?;
            start(topic_type, name)
        }
        CommandKind::Clarify { name } => clarify(&name),
    }
}

fn parse_topic_branch(branch: &str) -> Result<(&str, &str), String> {
    let (topic_type, name) = branch
        .split_once('/')
        .ok_or_else(|| format!("invalid branch `{branch}`; use `<type>/<name>`"))?;
    validate_name(topic_type, "topic type")?;
    validate_name(name, "change name")?;
    Ok((topic_type, name))
}

fn init() -> Result<(), String> {
    let root = PathBuf::from(git("rev-parse", &["--show-toplevel"])?);
    fs::create_dir_all(root.join("docs/agents/plans"))
        .map_err(|error| format!("create planning directory: {error}"))?;
    fs::create_dir_all(root.join(".worktrees"))
        .map_err(|error| format!("create worktree directory: {error}"))?;
    add_exclude(&root)?;
    println!("initialized Acta in {}", root.display());
    Ok(())
}

fn start(topic_type: &str, name: &str) -> Result<(), String> {
    validate_name(topic_type, "topic type")?;
    validate_name(name, "change name")?;

    let root = PathBuf::from(git("rev-parse", &["--show-toplevel"])?);
    let original_branch = git("rev-parse", &["--abbrev-ref", "HEAD"])?;
    if original_branch == "HEAD" {
        return Err("cannot start a plan from a detached HEAD".into());
    }

    let branch = format!("{topic_type}/{name}");
    if git_status("show-ref", &["--verify", &format!("refs/heads/{branch}")])?
        .status
        .success()
    {
        return Err(format!("branch `{branch}` already exists"));
    }
    let worktree = root.join(".worktrees").join(&branch);
    if worktree.exists() {
        return Err(format!(
            "worktree path `{}` already exists",
            worktree.display()
        ));
    }

    // Git Flow owns topic configuration and branch creation.
    git("flow", &[topic_type, "start", name])?;
    git("checkout", &[&original_branch]).map_err(|error| {
        format!("created `{branch}`, but could not restore `{original_branch}`: {error}")
    })?;

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

fn clarify(name: &str) -> Result<(), String> {
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

fn add_exclude(root: &Path) -> Result<(), String> {
    let common = PathBuf::from(git("rev-parse", &["--git-common-dir"])?);
    let common = if common.is_absolute() {
        common
    } else {
        root.join(common)
    };
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
    writeln!(file, "# begin acta\n/.worktrees/\n# end acta")
        .map_err(|error| format!("write Git exclude file: {error}"))
}

fn write_new(path: &Path, contents: &str) -> Result<(), String> {
    OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .and_then(|mut file| file.write_all(contents.as_bytes()))
        .map_err(|error| format!("write `{}`: {error}", path.display()))
}

fn validate_name(value: &str, label: &str) -> Result<(), String> {
    if value.is_empty()
        || value == "."
        || value == ".."
        || value.starts_with('-')
        || value.chars().any(|character| {
            character.is_whitespace()
                || !matches!(character, 'a'..='z' | '0'..='9' | '-' | '_' | '.')
        })
    {
        return Err(format!(
            "invalid {label} `{value}`; use lowercase letters, numbers, `-`, `_`, or `.`"
        ));
    }
    Ok(())
}

fn git(command: &str, args: &[&str]) -> Result<String, String> {
    let output = git_status(command, args)?;
    if !output.status.success() {
        return Err(command_error(command, args, &output));
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_owned())
}

fn git_status(command: &str, args: &[&str]) -> Result<Output, String> {
    Command::new("git")
        .arg(command)
        .args(args)
        .output()
        .map_err(|error| format!("run `git {command}`: {error}"))
}

fn command_error(command: &str, args: &[&str], output: &Output) -> String {
    let details = String::from_utf8_lossy(&output.stderr).trim().to_owned();
    if details.is_empty() {
        format!("`git {command} {}` failed", args.join(" "))
    } else {
        format!("`git {command} {}` failed: {details}", args.join(" "))
    }
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

    #[test]
    fn topic_branch_is_split_into_type_and_name() {
        assert_eq!(
            super::parse_topic_branch("feat/mytask"),
            Ok(("feat", "mytask"))
        );
        assert!(super::parse_topic_branch("mytask").is_err());
        assert!(super::parse_topic_branch("feat/my/task").is_err());
    }
}
