use clap::Parser;
use tempura::Cli;

fn main() {
    let cli = Cli::parse();

    println!("{:?}", cli);
}
