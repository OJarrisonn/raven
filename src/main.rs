use anyhow::Result;
use clap::Parser;
use rv_raven::{cli::{Cli, Subcommands}, config::Config, raven::{mailbox, send}};

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = Config::load()?;

    match cli.commands {
        Subcommands::Send { to, port, message } => send::send(&to, port, message),
        Subcommands::SendFile { to, port, file } => send::send_file(&to, port, file),
        Subcommands::Mailbox { commands } => mailbox::manage(commands, config),
    }
}
