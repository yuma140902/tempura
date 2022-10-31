use std::{
    borrow::{Borrow, Cow},
    collections::HashMap,
    fs, io,
    path::{Path, PathBuf},
};

use generator::Generator;
use project_config::GeneratorRule;
use walkdir::WalkDir;

use crate::project_config::ProjectConfig;

pub mod cli;
pub mod directory;
pub mod generator;
pub mod project_config;

fn single_generate(
    filepath: PathBuf,
    project_root: impl AsRef<Path>,
    rule: Option<&GeneratorRule>,
    generators: &HashMap<String, Box<dyn Generator>>,
) -> io::Result<()> {
    let rule = if let Some(rule) = rule {
        Cow::Borrowed(rule)
    } else {
        Cow::Owned(GeneratorRule::default())
    };

    let pages_directory = directory::get_pages_directory(&project_root);
    let mut output_directory = directory::get_output_directory(&project_root);
    if let Some(ref export_base) = rule.export_base {
        output_directory = output_directory.join(export_base);
    }

    let mut output_filepath =
        output_directory.join(directory::get_relative_path(&filepath, pages_directory));
    if let Some(ref export_extension) = rule.export_extension {
        output_filepath.set_extension(export_extension);
    }
    println!(
        "generating {} -> {}",
        filepath.display(),
        output_filepath.display()
    );

    let generator = generators.get(&rule.generator).unwrap();
    generator.generate(&filepath, &output_filepath, &rule);

    Ok(())
}

pub fn generate(project_root: impl AsRef<Path>) -> io::Result<()> {
    let pages_directory = directory::get_pages_directory(&project_root);
    let project_config_path = directory::get_project_config_path(&project_root);
    let config = {
        let mut config: ProjectConfig =
            serde_json::from_str(&fs::read_to_string(project_config_path)?)?;
        config.generate_regex();
        config
    };

    let generators = generator::get_generators(&project_root);

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

        let result = single_generate(filepath, &project_root, selected_rule, &generators);
        if let Err(err) = result {
            eprintln!("error: {:?}", err);
        }
    }
    println!("Done.");
    Ok(())
}
