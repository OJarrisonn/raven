use anyhow::Result;
use clap::Parser;
use cli::{Cli, Subcommands};
use config::Config;
use raven::{mailbox, receive, send};

mod cli;
mod config;
mod error;
mod raven;
mod util;

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = Config::load()?;

    match cli.commands {
        Subcommands::Receive { from, port } => receive::receive(
            from.unwrap_or(config.receiver.address.clone()),
            port.unwrap_or(config.receiver.port),
            config,
        ),
        Subcommands::Send { to, port, message } => send::send(&to, port, message),
        Subcommands::SendFile { to, port, file } => send::send_file(&to, port, file),
        Subcommands::Mailbox { commands } => mailbox::manage(commands, config),
    }
}
