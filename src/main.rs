use std::error::Error;

use clap::Parser;
use cli::Cli;
use config::RavenConfig;
use regex::Regex;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::signal;
use tokio::{io::AsyncReadExt, net::TcpListener};
use util::{assert_ipv4_address, RavenError};

mod cli;
mod consts;
mod config;
mod util;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let res = match cli.commands {
        cli::Subcommands::Receive { from, port } => {
            if assert_ipv4_address(&from) {
                receive(format!("{}:{}", from, port)).await
            } else {
                eprintln!("[RECEIVE]: Invalid address format: {}", from);
                Ok(())
            }
        },
        cli::Subcommands::Send { to, port, message } => send(to, port, message).await,
    };

    return res;
}

/// Function to start a receiving client
async fn receive(address: String) -> Result<(), Box<dyn Error>> {
    if util::assert_ipv4_address(&address) {
        eprintln!("[RECEIVE]: Invalid address format: {}", address);
        return Err(RavenError::InvalidAddress(address).into());
    }

    let listener = TcpListener::bind(&address);
    let config = RavenConfig::load_or_create(".raven.conf".into())?;
    let listener = listener.await?;

    println!("[RECEIVE]: Listening on {}", &address);

    loop {
        tokio::select! {
            _ = signal::ctrl_c() => {
                config.save()?;
                break;
            }
            res = listener.accept() => {
                match res {
                    Ok((mut socket, _)) => {
                        let mut buf = String::new();
                        let _ = socket.read_to_string(&mut buf).await?;
                        println!("[RECEIVE]: {} :: {}", socket.peer_addr().map(|addr| addr.to_string()).unwrap_or("unknown".into()), buf);
                    }
                    Err(e) => {
                        eprintln!("[RECEIVE]: Failed to accept connection: {}", e);
                    }
                }
            }
        }
    }

    Ok(())
} 

async fn send(address: String, port: u16, message: String) -> Result<(), Box<dyn Error>> {
    let config = RavenConfig::load_or_create(".raven.conf".into())?;

    let address = if util::assert_ipv4_address(&address) { 
        format!("{}:{}", address, port) 
    } else {
        match config.known_feathers.iter().find(|f| f.alias == address) {
            Some(feather) => format!("{}:{}", &feather.host, &feather.port),
            None => {
                eprintln!("[SEND]: Unknown feather alias: {}", address);
                return Ok(());
            }
        }
    };

    let mut stream = TcpStream::connect(&address).await?;

    let write = stream.write(message.as_bytes());
    
    println!("[SEND]: Sending to {}", &address);

    let len = write.await?;

    println!("[SEND]: Sent {len} bytes to {address}");

    Ok(())
}
