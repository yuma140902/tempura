use std::{fs, io, path::Path};

use anyhow::Context;

use crate::project_config::ProjectConfig;

pub mod cli;
pub mod directory;
pub mod handlebars_helpers;
mod loader;
pub mod pipeline;
pub mod project_config;
mod template_engine;
mod value;

pub use loader::*;
pub use template_engine::*;
pub use value::*;

#[tracing::instrument]
pub fn build(project_root: &Path) -> anyhow::Result<()> {
    let project_config_path = directory::get_project_config_path(project_root);
    let config: ProjectConfig = serde_yaml::from_str(
        &fs::read_to_string(&project_config_path)
            .with_context(|| format!("could not load file {}", project_config_path.display()))?,
    )
    .with_context(|| {
        format!(
            "failed to parse project config {}",
            project_config_path.display()
        )
    })?;

    dbg!(config);

    Ok(())
}

#[tracing::instrument]
pub fn init(project_root: &Path) -> io::Result<()> {
    println!("not implemented yet");

    Ok(())
}
