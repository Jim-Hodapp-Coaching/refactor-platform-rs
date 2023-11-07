use clap::{ArgGroup, Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, PartialEq, Eq, Debug)]
pub enum Commands {
    #[command(author, version, about, long_about = None)]
    Init
}