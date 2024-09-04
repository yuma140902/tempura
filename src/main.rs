use clap::Parser;
use tempura::cli::{Cli, Commands};
use tracing::debug;

const fn verbose_to_level(verbose: u8) -> tracing::Level {
    if verbose == 0 {
        #[cfg(debug_assertions)]
        return tracing::Level::DEBUG;
        #[cfg(not(debug_assertions))]
        return tracing::Level::INFO;
    } else if verbose == 1 {
        tracing::Level::DEBUG
    } else {
        tracing::Level::TRACE
    }
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(verbose_to_level(cli.verbose))
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    debug!("{:?}", cli);

    match &cli.command {
        Commands::Init { directory } => tempura::init(directory)?,
        Commands::Build { directory, .. } => tempura::build(directory)?,
    }

    Ok(())
}
