use anyhow::Result;
use clap::{Parser, Subcommand};

use gepetto::commands::scaffold_project;
use gepetto::io::print_welcome_message;

#[derive(Parser)]
#[command(name = "gepetto")]
#[command(about = "Solana's pinnochio companion")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new Pinocchio project
    New {
        /// Package name (optional, will prompt if not provided)
        name: Option<String>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::New { name }) => {
            scaffold_project(name).await?;
        }
        None => {
            print_welcome_message();
        }
    }

    Ok(())
}
