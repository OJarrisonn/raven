use clap::Parser;
use cli::Cli;

mod cli;
mod consts;

fn main() {
    let cli = Cli::parse();

    match cli.commands {
        cli::Subcommands::Receive { address } => println!("[NOT] Listening on {address}"),
        cli::Subcommands::Send { address, message } => println!("[NOT] Sending `{message}` to {address}")
    }
}
