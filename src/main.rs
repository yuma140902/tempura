use std::{fs, io};

use clap::Parser;
use tempura::{Cli, Commands};

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init { directory } => {
            let markdowns = directory.join("markdowns");
            let templates = directory.join("templates");
            fs::create_dir_all(&markdowns)?;
            println!("setup directory: {}", markdowns.to_string_lossy());
            fs::create_dir_all(&templates)?;
            println!("setup directory: {}", templates.to_string_lossy());
        }
        Commands::Gen {} => {}
    }

    println!("{:?}", cli);

    Ok(())
}
