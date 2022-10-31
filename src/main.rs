use std::io;

use clap::Parser;
use tempura::cli::{Cli, Commands};
use tracing::debug;

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
        Commands::Init { directory } => tempura::init(directory)?,
        Commands::Build { directory } => tempura::build(directory)?,
    }

    Ok(())
}
