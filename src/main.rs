use std::{error::Error, io::Write, net::TcpStream};

use clap::Parser;
use cli::Cli;
use tokio::{io::AsyncReadExt, net::TcpListener};

mod cli;
mod consts;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match cli.commands {
        cli::Subcommands::Receive { address } => receive(address).await,
        cli::Subcommands::Send { address, message } => send(address, message)
    }
}

/// Function to start a receiving client
async fn receive(address: String) -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind(address).await?;

    loop {
        let (mut socket, _) = listener.accept().await?;
        let mut buf = String::new();
        let _ = socket.read_to_string(&mut buf).await?;
        println!("{}", buf);
    }

    Ok(())
} 

fn send(address: String, message: String) -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect(&address)?;

    let len = stream.write(message.as_bytes())?;

    println!("Sent {len} bytes to {address}");

    Ok(())
}