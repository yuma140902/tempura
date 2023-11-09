use std::{fs, io, path::Path};

use tracing::info;

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
pub fn build(project_root: &Path) -> io::Result<()> {
    let project_config_path = directory::get_project_config_path(project_root);
    let config: ProjectConfig = serde_json::from_str(&fs::read_to_string(project_config_path)?)?;

    dbg!(config);

    Ok(())
}

#[tracing::instrument]
pub fn init(project_root: &Path) -> io::Result<()> {
    let pages = directory::get_pages_directory(project_root);
    fs::create_dir_all(&pages)?;
    fs::write(
        pages.join("sample.md"),
        include_str!("../resources/sample.md"),
    )?;
    fs::write(
        pages.join("style.css"),
        include_str!("../resources/style.css"),
    )?;
    fs::create_dir_all(pages.join("sub_dir"))?;
    fs::write(
        pages.join("sub_dir/sample2.md"),
        include_str!("../resources/sample2.md"),
    )?;
    info!("setup pages directory: {}", pages.display());

    let templates = directory::get_templates_directory(project_root);
    fs::create_dir_all(&templates)?;
    fs::write(
        templates.join("page.html.hbs"),
        include_str!("../resources/page.html.hbs"),
    )?;
    info!("setup templates directory: {}", templates.display());

    let output = directory::get_output_directory(project_root);
    fs::create_dir_all(&output)?;
    info!("setup output directory: {}", output.display());

    let config_file = directory::get_project_config_path(project_root);
    let config = ProjectConfig::default();
    let config_json = serde_json::to_string_pretty(&config)?;
    fs::write(&config_file, config_json)?;
    info!("setup project config file: {}", config_file.display());

    let gitignore = project_root.join(".gitignore");
    fs::write(&gitignore, include_str!("../resources/gitignore"))?;
    info!("setup .gitignore file: {}", gitignore.display());

    info!("setup done.");

    println!();
    println!("Setup done. To build website, run:");
    println!();
    println!("  cd {}", project_root.display());
    println!("  tempura build .");
    println!();

    Ok(())
}
