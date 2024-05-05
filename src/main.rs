use std::error::Error;
use std::future::IntoFuture;

use clap::Parser;
use cli::Cli;
use config::RavenConfig;
use consts::get_config_file_name;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::signal;
use tokio::{io::AsyncReadExt, net::TcpListener};

mod cli;
mod consts;
mod config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let res = match cli.commands {
        cli::Subcommands::Receive { address } => receive(address).await,
        cli::Subcommands::Send { address, message } => send(address, message).await
    };

    return res;
}

/// Function to start a receiving client
async fn receive(address: String) -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind(address).await?;
    let config = RavenConfig::load_or_create(get_config_file_name())?;

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
                        println!("{}", buf);
                    }
                    Err(e) => {
                        eprintln!("Failed to accept connection: {}", e);
                    }
                }
            }
        }
    }

    Ok(())
} 

async fn send(address: String, message: String) -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect(&address).await?;

    let write = stream.write(message.as_bytes());
    
    println!("Sending to {}", &address);

    let len = write.await?;

    println!("Sent {len} bytes to {address}");

    Ok(())
}
