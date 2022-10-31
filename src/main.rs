use std::{fs, io};

use clap::Parser;
use tempura::{
    cli::{Cli, Commands},
    directory, generate,
    project_config::ProjectConfig,
};

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    #[cfg(debug_assertions)]
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
            println!("setup pages directory: {}", pages.display());

            let templates = directory.join("src").join("templates");
            fs::create_dir_all(&templates)?;
            fs::write(
                templates.join("page.html.hbs"),
                include_str!("../resources/page.html.hbs"),
            )?;
            println!("setup templates directory: {}", templates.display());

            let output = directory::get_output_directory(&directory);
            fs::create_dir_all(&output)?;
            println!("setup output directory: {}", output.display());

            let config_file = directory::get_project_config_path(&directory);
            let config = ProjectConfig::default();
            let config_json = serde_json::to_string_pretty(&config)?;
            fs::write(&config_file, config_json)?;
            println!("setup project config file: {}", config_file.display());

            println!("setup done.");
        }
        Commands::Build { directory } => generate(directory)?,
    }

    Ok(())
}
