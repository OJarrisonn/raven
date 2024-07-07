use clap::{Parser, Subcommand};
use crate::consts::{LISTEN_DEFAULT_ADDRESS, LISTEN_DEFAULT_PORT};

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
        from: String,
        #[arg(short, long, value_name = "PORT", default_value_t = LISTEN_DEFAULT_PORT.into())]
        port: u16
    },
    /// Sends a message by a raven to another client
    Send {
        /// The raven's destination
        #[arg(short, long, value_name = "DESTINATION")]
        to: String,
        #[arg(short, long, value_name = "PORT", default_value_t = LISTEN_DEFAULT_PORT.into())]
        port: u16,
       /// The message the raven must send
        #[arg(value_name = "MESSAGE")]
        message: String
    }
}