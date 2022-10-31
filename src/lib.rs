use std::{
    borrow::Borrow,
    fs, io,
    path::{Path, PathBuf},
};

use clap::{Parser, Subcommand};
use handlebars::{handlebars_helper, Handlebars};
use serde_json::{Map, Value as Json};
use serde_yaml::Value as Yaml;
use walkdir::WalkDir;

pub mod directory;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Init {
        #[arg(default_value = ".")]
        directory: PathBuf,
    },
    Gen {
        #[arg(default_value = ".")]
        directory: PathBuf,
    },
}

pub fn split_frontmatter(text: &str) -> (Option<String>, String) {
    if let Some((front, body)) = matter::matter(text) {
        (Some(front), body)
    } else {
        (None, text.to_owned())
    }
}

handlebars_helper!(md2html: |markdown: String| {
    let mut options = pulldown_cmark::Options::empty();
    let parser = pulldown_cmark::Parser::new_ext(&markdown, options);
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);
    html_output
});

fn make_data(yaml: &Yaml, markdown: String) -> Map<String, Json> {
    let mut data = Map::new();

    data.insert("markdown".to_owned(), handlebars::to_json(markdown));
    data.insert("front".to_owned(), handlebars::to_json(yaml));

    data
}

fn single_generate(
    filepath: PathBuf,
    handlebars: &Handlebars,
    pages_directory: impl AsRef<Path>,
    output_directory: impl AsRef<Path>,
) -> io::Result<()> {
    let output_directory = output_directory.as_ref().to_path_buf();
    let mut output_filepath =
        output_directory.join(get_output_relative_path(&filepath, pages_directory));
    output_filepath.set_extension("html");
    println!(
        "processing {} -> {}",
        filepath.display(),
        output_filepath.display()
    );

    let content = fs::read_to_string(filepath)?;
    let (maybe_yaml, markdown) = split_frontmatter(&content);
    let yaml = maybe_yaml.unwrap();
    let yaml: Yaml = serde_yaml::from_str(&yaml).unwrap();

    let data = make_data(&yaml, markdown);

    let html_output = handlebars
        .render(yaml["template"].as_str().unwrap(), &data)
        .unwrap();

    if let Some(parent) = output_filepath.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&output_filepath, html_output)?;

    Ok(())
}

fn get_output_relative_path(file: impl AsRef<Path>, pages_directory: impl AsRef<Path>) -> PathBuf {
    pathdiff::diff_paths(file, pages_directory).unwrap()
}

fn get_template_relative_path(
    file: impl AsRef<Path>,
    templates_directory: impl AsRef<Path>,
) -> PathBuf {
    pathdiff::diff_paths(file, templates_directory).unwrap()
}

fn setup_handlebars(templates_directory: impl AsRef<Path>) -> Handlebars<'static> {
    let mut handlebars = Handlebars::new();
    handlebars.register_helper("md2html", Box::new(md2html));

    for path in WalkDir::new(&templates_directory)
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|e| e.path().to_path_buf())
        .filter(|p| p.is_file() && p.extension().map(|ext| ext == "hbs").unwrap_or(false))
    {
        let relative_path = get_template_relative_path(&path, &templates_directory);
        handlebars
            .register_template_file(relative_path.to_string_lossy().borrow(), path)
            .unwrap();
        println!("registered template: {}", relative_path.display());
    }

    handlebars
}

pub fn generate(root_directory: impl AsRef<Path>) -> io::Result<()> {
    let pages_directory = directory::get_pages_directory(&root_directory);
    let templates_directory = directory::get_templates_directory(&root_directory);
    let output_directory = directory::get_output_directory(&root_directory);

    let handlebars = setup_handlebars(templates_directory);

    for filepath in WalkDir::new(&pages_directory)
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|e| e.path().to_path_buf())
        .filter(|p| p.is_file() && p.extension().map(|ext| ext == "md").unwrap_or(false))
    {
        let result = single_generate(filepath, &handlebars, &pages_directory, &output_directory);
        if let Err(err) = result {
            eprintln!("error: {:?}", err);
        }
    }
    println!("Done.");
    Ok(())
}
