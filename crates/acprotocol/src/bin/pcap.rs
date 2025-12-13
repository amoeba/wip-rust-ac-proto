use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod read;
mod tui;

#[derive(Parser)]
#[command(name = "pcap")]
#[command(about = "Parse .pcap network capture files", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Read and print packet data from a pcap file
    Read {
        /// Path to .pcap file
        path: PathBuf,
    },
    /// Interactive view for browsing pcap files
    View {
        /// Path to .pcap file
        path: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Read { path } => read::run(&path)?,
        Commands::View { path } => tui::run(&path)?,
    }

    Ok(())
}
