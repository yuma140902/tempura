use std::{
    io,
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

fn single_generate(filepath: PathBuf, output_directory: impl AsRef<Path>) -> io::Result<()> {
    let output_directory = output_directory.as_ref().to_path_buf();
    let output_filepath = output_directory.join(&filepath);
    println!(
        "processing {} -> {}",
        filepath.display(),
        output_filepath.display()
    );
    Ok(())
}

fn get_output_relative_path(
    file: impl AsRef<Path>,
    _pages_directory: impl AsRef<Path>,
) -> io::Result<PathBuf> {
    if file.as_ref().is_relative() {
        Ok(file.as_ref().to_path_buf())
    } else {
        todo!()
    }
}

pub fn generate(root_directory: impl AsRef<Path>) -> io::Result<()> {
    let pages_directory = get_pages_directory(&root_directory);
    let output_directory = get_output_directory(&root_directory);

    for filepath in WalkDir::new(pages_directory)
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|e| e.path().to_path_buf())
        .filter(|p| p.is_file() && p.extension().map(|ext| ext == "md").unwrap_or(false))
    {
        let result = single_generate(filepath, &output_directory);
        if let Err(err) = result {
            eprintln!("error: {:?}", err);
        }
    }
    println!("Done.");
    Ok(())
}
