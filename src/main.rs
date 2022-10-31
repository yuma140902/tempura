use std::{fs, io};

use clap::Parser;
use tempura::{generate, Cli, Commands};

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    println!("{:?}", cli);

    match &cli.command {
        Commands::Init { directory } => {
            let pages = tempura::get_pages_directory(&directory);
            fs::create_dir_all(&pages)?;
            println!("setup pages directory: {}", pages.to_string_lossy());

            let templates = tempura::get_templates_directory(&directory);
            fs::create_dir_all(&templates)?;
            println!("setup templates directory: {}", templates.to_string_lossy());

            let output = tempura::get_output_directory(&directory);
            fs::create_dir_all(&output)?;
            println!("setup output directory: {}", output.to_string_lossy());

            println!("setup done.");
        }
        Commands::Gen { directory } => generate(directory)?,
    }

    Ok(())
}
