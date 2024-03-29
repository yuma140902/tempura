use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, help_template = "\
{before-help}{name} {version} 
{tab}by {author}
{about-with-newline}
{usage-heading} {usage}

{all-args}{after-help}
")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(
        short = 'v',
        action = clap::ArgAction::Count,
        global = true,
        help = "`-v` to show debug log, `-vv` to show trace log",
    )]
    pub verbose: u8,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Initializes a Tempura project
    Init {
        #[arg(default_value = ".")]
        directory: PathBuf,
    },
    /// Builds website
    Build {
        #[arg(default_value = ".")]
        directory: PathBuf,
    },
}
