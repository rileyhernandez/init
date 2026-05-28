use crate::args::Language;
use anyhow::{Result, anyhow};
use dialoguer::{Input, Select, theme::ColorfulTheme};

/// Renders an interactive terminal menu using arrow keys to select a target language.
///
/// Returns an error if the terminal interface fails to capture the user's selection.
pub fn prompt_for_language() -> Result<Language> {
    let languages = vec!["Python", "Rust"];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose a language")
        .items(&languages)
        .default(0)
        .interact()
        .map_err(|e| anyhow!("Failed to read selection: {e}"))?;

    match selection {
        0 => Ok(Language::Python),
        1 => Ok(Language::Rust),
        _ => Err(anyhow!("Invalid selection index")),
    }
}

/// Prompts the user via terminal text input to specify a target Distrobox container name.
///
/// Automatically enforces non-empty inputs and returns an error if terminal reading fails.
pub fn prompt_for_distrobox() -> Result<String> {
    let input: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter distrobox name")
        .interact_text()
        .map_err(|e| anyhow!("Failed to read input: {e}"))?;

    Ok(input.trim().to_string())
}

/// Prompts the user via terminal text input for a project name.
pub fn prompt_for_project_name() -> Result<String> {
    let input: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter project name")
        .interact_text()
        .map_err(|e| anyhow!("Failed to read project name: {e}"))?;

    Ok(input.trim().to_string())
}
