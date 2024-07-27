use anyhow::Result;
use clap::Parser;
use rv_raven::config::Config;
use cli::{Cli, Subcommands};

mod cli;
mod send;
mod mailbox;

fn main() -> Result<()> {
    //let cli = Cli::parse();
    let config = Config::load()?;

    match (Subcommands::Send { to: "127.0.0.1".into(), port: 12345, message: "Hello World".into() }) {
        Subcommands::Send { to, port, message } => send::send(&config, &to, port, &message),
        Subcommands::SendFile { to: _, port: _, file: _ } => todo!(), //send::send_file(&to, port, file),
        Subcommands::Mailbox { commands } => mailbox::manage(commands, &config),
    }
}
