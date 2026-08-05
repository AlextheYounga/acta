use clap::{Parser, Subcommand};

use crate::commands::agentsmd::agentsmd;
use crate::commands::clarify::clarify;
use crate::commands::init::init;
use crate::commands::start::start;
use crate::utils::validate_name;

#[derive(Parser)]
#[command(name = "acta", about = "Create and maintain lightweight change plans")]
pub struct Cli {
    #[command(subcommand)]
    command: CommandKind,
}

#[derive(Subcommand)]
enum CommandKind {
    Agentsmd,
    Init,
    Start { branch: String },
    Clarify { name: String },
}

pub fn run(cli: Cli) -> Result<(), String> {
    match cli.command {
        CommandKind::Agentsmd => agentsmd(),
        CommandKind::Init => init(),
        CommandKind::Start { branch } => {
            let (topic_type, name) = parse_topic_branch(&branch)?;
            start(topic_type, name)
        }
        CommandKind::Clarify { name } => clarify(&name),
    }
}

pub fn parse() -> Cli {
    Cli::parse()
}

fn parse_topic_branch(branch: &str) -> Result<(&str, &str), String> {
    let (topic_type, name) =
        branch.split_once('/').ok_or_else(|| format!("invalid branch `{branch}`; use `<type>/<name>`"))?;
    validate_name(topic_type, "topic type")?;
    validate_name(name, "change name")?;
    Ok((topic_type, name))
}

#[cfg(test)]
mod tests {
    use super::parse_topic_branch;

    #[test]
    fn topic_branch_is_split_into_type_and_name() {
        assert_eq!(parse_topic_branch("feat/mytask"), Ok(("feat", "mytask")));
        assert!(parse_topic_branch("mytask").is_err());
        assert!(parse_topic_branch("feat/my/task").is_err());
    }
}
