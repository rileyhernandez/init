use clap::{Parser, ValueEnum};
use serde::Deserialize;

/// Command-line arguments for configuring a development distrobox environment.
#[derive(Parser, Debug)]
#[command(name = "devbox-init")]
#[command(version, about = "Sets up development distroboxes", long_about = None)]
pub struct Args {
    /// The targeted programming language environment.
    #[arg(short, long)]
    pub language: Option<Language>,

    /// The name of the target Distrobox container.
    #[arg(short, long)]
    pub distrobox: Option<String>,

    /// The name of the new project.
    #[arg(short, long)]
    pub project_name: Option<String>,
}

/// Supported programming languages for environment initialization.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    Python,
    Rust,
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Python => write!(f, "Python"),
            Self::Rust => write!(f, "Rust"),
        }
    }
}
