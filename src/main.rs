// Copyright (C) 2026 Riley Hernandez
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.


mod args;
mod io;

use crate::args::{Args, Language};
use anyhow::{anyhow, Result};
use clap::Parser;
use handlebars::Handlebars;
use spinners::{Spinner, Spinners};
use std::collections::HashMap;
use std::path::Path;
use xshell::{cmd, Shell};

const JUSTFILE_PYTHON_TEMPLATE: &str = include_str!("../templates/python/justfile.tmpl");
const JUSTFILE_RUST_TEMPLATE: &str = include_str!("../templates/rust/justfile.tmpl");
const ZED_PYTHON_TEMPLATE: &str = include_str!("../templates/python/zed_settings.json.tmpl");
const ZED_RUST_TEMPLATE: &str = include_str!("../templates/rust/zed_settings.json.tmpl");
const PYPROJECT_TEMPLATE: &str = include_str!("../templates/python/pyproject.toml.tmpl");

/// Entry point of the utility. Collects validation criteria through flags 
/// or active shell queries, generating isolated language dev configs.
fn main() -> Result<()> {
    let cli_args = Args::parse();

    let language = match cli_args.language {
        Some(lang) => lang,
        None => io::prompt_for_language()?,
    };

    let distrobox = match cli_args.distrobox {
        Some(dbx) => dbx,
        None => io::prompt_for_distrobox()?,
    };

    let project_name = match cli_args.project_name {
        Some(dir) => dir,
        None => io::prompt_for_project_name()?,
    };

    let mut sp = Spinner::new(
        Spinners::Mindblown,
        format!("Generating configuration files for {language} in '{project_name}'..."),
    );

    if let Err(e) = generate_configs(&project_name, &distrobox, language) {
        sp.stop();
        return Err(anyhow!("Failed to generate environment configurations: {e}"));
    }

    sp.stop();
    println!("\n🚀 Environment successfully initialized!");

    Ok(())
}

/// Parses local context attributes into static templates and exports configuration mappings.
///
/// Evaluates settings for build orchestrators, local toolchains, and editor setups.
fn generate_configs(project_name: &str, container: &str, lang: Language) -> Result<()> {
    let mut reg = Handlebars::new();
    
    let (just_tmpl, zed_tmpl, lsp_name) = match lang {
        Language::Rust => (JUSTFILE_RUST_TEMPLATE, ZED_RUST_TEMPLATE, "rust-analyzer"),
        Language::Python => {
            reg.register_template_string("pyproject", PYPROJECT_TEMPLATE)?;
            (JUSTFILE_PYTHON_TEMPLATE, ZED_PYTHON_TEMPLATE, "ruff")
        }
    };
    
    reg.register_template_string("justfile", just_tmpl)?;
    reg.register_template_string("zed", zed_tmpl)?;
    
    let mut data = HashMap::new();
    data.insert("container_name", container);
    data.insert("project_name", project_name);
    data.insert("lsp_name", lsp_name);

    let rendered_justfile = reg.render("justfile", &data)?;
    let rendered_zed = reg.render("zed", &data)?;

    let base_path = Path::new(project_name);
    let sh = Shell::new()?;

    match lang {
        Language::Python => {
            cmd!(sh, "uv init --package {project_name}").run()?;            
            let rendered_pyproject = reg.render("pyproject", &data)?;
            let pyproject_path = base_path.join("pyproject.toml");
            std::fs::write(&pyproject_path, rendered_pyproject)?;
        }
        Language::Rust => {
            cmd!(sh, "cargo new {project_name}").run()?;
        }
    }    

    let justfile_path = base_path.join("justfile");
    std::fs::write(&justfile_path, rendered_justfile)?;
    
    let zed_dir = base_path.join(".zed");
    std::fs::create_dir_all(&zed_dir)?;
    
    let zed_settings_path = zed_dir.join("settings.json");
    std::fs::write(&zed_settings_path, rendered_zed)?;

    Ok(())
}