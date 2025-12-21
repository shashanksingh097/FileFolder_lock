use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "locker",
    version = "0.1.0",
    about = "Secure file & folder locker with destructive security"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Lock a file or folder (DESTRUCTIVE)
    Lock {
        /// Path to file or folder
        path: String,

        /// Maximum wrong password attempts
        #[arg(short, long, default_value = "3")]
        attempts: u8,

        /// Skip confirmation (DANGEROUS)
        #[arg(long)]
        force: bool,
    },

    /// Unlock a file or folder
    Unlock {
        path: String,
    },
}
