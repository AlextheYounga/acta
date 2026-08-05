pub fn git(command: &str, args: &[&str]) -> Result<String, String> {
    let output = git_status(command, args)?;
    if !output.status.success() {
        return Err(command_error(command, args, &output));
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_owned())
}

pub fn git_status(command: &str, args: &[&str]) -> Result<Output, String> {
    Command::new("git").arg(command).args(args).output().map_err(|error| format!("run `git {command}`: {error}"))
}

fn command_error(command: &str, args: &[&str], output: &Output) -> String {
    let details = String::from_utf8_lossy(&output.stderr).trim().to_owned();
    if details.is_empty() {
        format!("`git {command} {}` failed", args.join(" "))
    } else {
        format!("`git {command} {}` failed: {details}", args.join(" "))
    }
}
use std::process::{Command, Output};
