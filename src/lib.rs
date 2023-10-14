use std::{
    borrow::{Borrow, Cow},
    collections::HashMap,
    fs, io,
    path::{Path, PathBuf},
};

use generator::Generator;
use project_config::GeneratorRule;
use tracing::{error, info};
use walkdir::WalkDir;

use crate::project_config::ProjectConfig;

pub mod cli;
pub mod directory;
pub mod generator;
mod loader;
pub mod project_config;
mod value;

pub use loader::*;
pub use value::*;

fn build_single_file(
    filepath: PathBuf,
    project_root: &Path,
    rule: Option<&GeneratorRule>,
    generators: &HashMap<String, Box<dyn Generator>>,
) -> io::Result<()> {
    let rule = if let Some(rule) = rule {
        Cow::Borrowed(rule)
    } else {
        Cow::Owned(GeneratorRule::default())
    };

    let pages_directory = directory::get_pages_directory(project_root);
    let mut output_directory = directory::get_output_directory(project_root);
    if let Some(ref export_base) = rule.export_base {
        output_directory = output_directory.join(export_base);
    }

    let mut output_filepath =
        output_directory.join(directory::get_relative_path(&filepath, pages_directory));
    if let Some(ref export_extension) = rule.export_extension {
        output_filepath.set_extension(export_extension);
    }
    info!(
        "generating {} from {} with generator '{}'",
        output_filepath.display(),
        filepath.display(),
        rule.generator,
    );

    let generator = generators.get(&rule.generator).unwrap();
    generator
        .generate(&filepath, &output_filepath, project_root.as_ref(), &rule)
        .unwrap();

    Ok(())
}

#[tracing::instrument]
pub fn build(project_root: &Path) -> io::Result<()> {
    let pages_directory = directory::get_pages_directory(project_root);
    let project_config_path = directory::get_project_config_path(project_root);
    let config = {
        let mut config: ProjectConfig =
            serde_json::from_str(&fs::read_to_string(project_config_path)?)?;
        config.generate_regex();
        config
    };

    let generators = generator::get_generators(project_root);

    for filepath in WalkDir::new(&pages_directory)
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|e| e.path().to_path_buf())
        .filter(|p| p.is_file())
    {
        let mut selected_rule = None;
        for rule in config.generator.rules.iter() {
            if rule
                .match_regex
                .get()
                .unwrap()
                .is_match(filepath.to_string_lossy().borrow())
            {
                selected_rule = Some(rule);
            }
        }

        let result = build_single_file(filepath, project_root, selected_rule, &generators);
        if let Err(err) = result {
            error!("error: {:?}", err);
        }
    }
    info!("Done.");
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

    info!("setup done.");

    Ok(())
}
