use rv_raven::util::LISTEN_DEFAULT_PORT;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(propagate_version = true)]
#[command(version)]
#[command(name = "raven")]
/// Raven is a CLI tool to share data across your devices in your local/private networks.
/// Instantiate a receiving end and then send text messages or files to it from your other devices in your local network.
/// Raven can be configured with `config.toml` in the raven home directory (either `$HOME/.raven` or `$RAVEN_HOME`).
pub struct Cli {
    #[command(subcommand)]
    pub commands: Subcommands,
}

#[derive(Subcommand)]
pub enum Subcommands {
    /// Sends a message by a raven to another client
    Send {
        /// The raven's destination address
        #[arg(long, value_name = "DESTINATION")]
        to: String,
        /// The port where the raven must arrive
        #[arg(short, long, value_name = "PORT", default_value_t = LISTEN_DEFAULT_PORT.into())]
        port: u16,
        /// The message the raven must send
        #[arg(value_name = "MESSAGE")]
        message: String,
    },

    /// Sends a file by a raven to another client
    SendFile {
        /// The raven's destination address
        #[arg(long, value_name = "DESTINATION")]
        to: String,
        /// The port where the raven must arrive
        #[arg(short, long, value_name = "PORT", default_value_t = LISTEN_DEFAULT_PORT.into())]
        port: u16,
        /// The file the raven must send
        #[arg(value_name = "FILE")]
        file: String,
    },
    /// Manages the mailbox with your received messages and files
    Mailbox {
        #[command(subcommand)]
        commands: MailboxSubcommands,
    },
}

#[derive(Subcommand)]
pub enum MailboxSubcommands {
    /// Lists the messages and files in the mailbox
    List {
        #[arg(short, long, default_value_t = false)]
        files: bool,
        #[arg(short, long, default_value_t = false)]
        messages: bool,
    },
    /// Deletes a message or file from the mailbox
    Delete {
        /// The index of the message or file to delete
        #[arg(value_name = "ID")]
        index: usize,
        #[arg(short, long, default_value_t = false)]
        file: bool,
        #[arg(short, long, default_value_t = false)]
        message: bool,
    },
    /// Opens a message or file from the mailbox
    Show {
        /// The index of the message or file to open
        #[arg(value_name = "ID")]
        index: usize,
        #[arg(short, long, default_value_t = false)]
        file: bool,
        #[arg(short, long, default_value_t = false)]
        message: bool,
    },
}
