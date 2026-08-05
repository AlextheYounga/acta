use std::path::Path;

use dialoguer::Select;

use crate::commands::AGENT_TEMPLATES;
use crate::utils::write_new;

pub fn agentsmd() -> Result<(), String> {
    let destination = Path::new("AGENTS.md");
    if destination.exists() {
        return Err("`AGENTS.md` already exists; refusing to overwrite it".into());
    }

    let choices: Vec<&str> = AGENT_TEMPLATES.iter().map(|(name, _)| *name).collect();
    let selection = Select::new()
        .with_prompt("Choose an AGENTS.md template")
        .items(&choices)
        .interact()
        .map_err(|error| format!("choose a template: {error}"))?;
    write_new(destination, AGENT_TEMPLATES[selection].1)?;
    println!("created AGENTS.md from {}", AGENT_TEMPLATES[selection].0);
    Ok(())
}
