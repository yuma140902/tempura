use std::{fs, io};

use clap::Parser;
use tempura::{
    build,
    cli::{Cli, Commands},
    directory,
    project_config::ProjectConfig,
};
use tracing::{debug, info};

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    #[cfg(debug_assertions)]
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::DEBUG)
        .finish();
    #[cfg(not(debug_assertions))]
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    debug!("{:?}", cli);

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
            info!("setup pages directory: {}", pages.display());

            let templates = directory.join("src").join("templates");
            fs::create_dir_all(&templates)?;
            fs::write(
                templates.join("page.html.hbs"),
                include_str!("../resources/page.html.hbs"),
            )?;
            info!("setup templates directory: {}", templates.display());

            let output = directory::get_output_directory(&directory);
            fs::create_dir_all(&output)?;
            info!("setup output directory: {}", output.display());

            let config_file = directory::get_project_config_path(&directory);
            let config = ProjectConfig::default();
            let config_json = serde_json::to_string_pretty(&config)?;
            fs::write(&config_file, config_json)?;
            info!("setup project config file: {}", config_file.display());

            info!("setup done.");
        }
        Commands::Build { directory } => build(directory)?,
    }

    Ok(())
}
