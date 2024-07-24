use std::{
    error::Error,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

use clap::Parser;
use cli::Cli;

mod cli;
mod util;

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match cli.commands {
        cli::Subcommands::Receive { from, port } => receive(from, port),
        cli::Subcommands::Send { to, port, message } => send(to, port, message),
    }
}

/// Opens the client for receiving messages from a raven
/// The receiver works in a loop, listening for incoming connections and printing the received message.
/// The receiver will listen on the provided ipv4 address(`from`) and `port`.
/// 
/// This function actually only returns an error if the connection fails to be established. Otherwise it will loop forever.
fn receive(from: String, port: u16) -> Result<(), Box<dyn Error>> {
    if !util::is_ipv4_address(&from) {
        return Err("Invalid address".into());
    }

    let listener = TcpListener::bind(format!("{}:{}", from, port))?;
    println!("Listening on {}:{}", from, port);

    for stream in listener.incoming() {
        let mut stream = stream?;
        println!("Connection established: {:?}", stream);

        let mut msg = String::new();
        stream.read_to_string(&mut msg)?;
        println!("Received message: {}", msg);
    }

    Ok(())
}

/// Sends a message by a raven to another client.
/// The target client is specified by the `to` ipv4 address and `port`. The message is a `String`.
/// It will send only one message and finishes, the TCP protocol will take care of the rest.
/// If the target is offline, the connection will fail and the function will return an error.
fn send(to: String, port: u16, message: String) -> Result<(), Box<dyn Error>> {
    if !util::is_ipv4_address(&to) {
        return Err("Invalid address".into());
    }

    let mut stream = TcpStream::connect(format!("{}:{}", to, port))?;
    println!("Connected to {}:{}", to, port);

    stream.write_all(message.as_bytes())?;
    println!("Message sent: {}", message);

    Ok(())
}
