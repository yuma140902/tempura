use std::{
    fs, io,
    path::{Path, PathBuf},
};

use clap::{Parser, Subcommand};
use walkdir::WalkDir;

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

pub fn get_pages_directory(root_directory: impl AsRef<Path>) -> PathBuf {
    root_directory.as_ref().join("pages")
}

pub fn get_templates_directory(root_directory: impl AsRef<Path>) -> PathBuf {
    root_directory.as_ref().join("templates")
}

pub fn get_output_directory(root_directory: impl AsRef<Path>) -> PathBuf {
    root_directory.as_ref().join("public")
}

pub fn split_frontmatter(text: &str) -> (Option<String>, String) {
    if let Some((front, body)) = matter::matter(text) {
        (Some(front), body)
    } else {
        (None, text.to_owned())
    }
}

fn single_generate(
    filepath: PathBuf,
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
    let (_maybe_yaml, markdown) = split_frontmatter(&content);

    let mut options = pulldown_cmark::Options::empty();
    let parser = pulldown_cmark::Parser::new_ext(&markdown, options);
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);

    if let Some(parent) = output_filepath.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&output_filepath, html_output)?;

    Ok(())
}

fn get_output_relative_path(file: impl AsRef<Path>, pages_directory: impl AsRef<Path>) -> PathBuf {
    pathdiff::diff_paths(file, pages_directory).unwrap()
}

pub fn generate(root_directory: impl AsRef<Path>) -> io::Result<()> {
    let pages_directory = get_pages_directory(&root_directory);
    let output_directory = get_output_directory(&root_directory);

    for filepath in WalkDir::new(&pages_directory)
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|e| e.path().to_path_buf())
        .filter(|p| p.is_file() && p.extension().map(|ext| ext == "md").unwrap_or(false))
    {
        let result = single_generate(filepath, &pages_directory, &output_directory);
        if let Err(err) = result {
            eprintln!("error: {:?}", err);
        }
    }
    println!("Done.");
    Ok(())
}
