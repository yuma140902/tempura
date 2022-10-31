use std::{fs, io};

use clap::Parser;
use tempura::{
    cli::{Cli, Commands},
    directory, generate,
};

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    println!("{:?}", cli);

    match &cli.command {
        Commands::Init { directory } => {
            let pages = directory::get_pages_directory(&directory);
            fs::create_dir_all(&pages)?;
            fs::write(
                pages.join("sample.md"),
                include_str!("../resources/sample.md"),
            )?;
            fs::create_dir_all(pages.join("sub_dir"))?;
            fs::write(
                pages.join("sub_dir/sample2.md"),
                include_str!("../resources/sample2.md"),
            )?;
            println!("setup pages directory: {}", pages.to_string_lossy());

            let templates = directory::get_templates_directory(&directory);
            fs::create_dir_all(&templates)?;
            fs::write(
                templates.join("page.html.hbs"),
                include_str!("../resources/page.html.hbs"),
            )?;
            println!("setup templates directory: {}", templates.to_string_lossy());

            let output = directory::get_output_directory(&directory);
            fs::create_dir_all(&output)?;
            println!("setup output directory: {}", output.to_string_lossy());

            println!("setup done.");
        }
        Commands::Build { directory } => generate(directory)?,
    }

    Ok(())
}
