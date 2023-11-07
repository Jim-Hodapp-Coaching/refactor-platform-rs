use clap::Parser;
use database_management::cli::{Cli, Commands};

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => {
            println!("hello world works!")
        }
    }
}