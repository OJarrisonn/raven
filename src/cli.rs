use clap::{Parser, Subcommand};
use crate::consts::LISTEN_DEFAULT_ADDRESS;

#[derive(Parser)]
#[command(propagate_version = true)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Subcommands
}

#[derive(Subcommand)]
pub enum Subcommands {
    /// Opens the client for receiving messages from a raven
    Receive {
        /// The address where to open the socket for the raven to arrive
        #[arg(short, long, value_name = "ADDRESS", default_value_t = LISTEN_DEFAULT_ADDRESS.into())]
        address: String,
    },
    /// Sends a message by a raven to another client
    Send {
        /// The address of the raven's destination
        #[arg(short, long, value_name = "ADDRESS")]
        address: String,
        /// The message the raven must send
        #[arg(value_name = "MESSAGE")]
        message: String
    }
}