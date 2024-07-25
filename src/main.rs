use std::{
    error::Error,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

use clap::Parser;
use cli::{Cli, Subcommands};
use config::Config;
use raven::Raven;
use util::basename;

mod cli;
mod config;
mod raven;
mod util;

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let config = Config::load()?;

    match cli.commands {
        Subcommands::Receive { from, port } => receive(
            from.unwrap_or(config.receiver.address.clone()), 
            port.unwrap_or(config.receiver.port),
            config
        ),
        Subcommands::Send { to, port, message } => send(&to, port, message),
        Subcommands::SendFile { to, port, file } => send_file(&to, port, file),
    }
}

/// Opens the client for receiving messages from a raven
/// The receiver works in a loop, listening for incoming connections and printing the received message.
/// The receiver will listen on the provided ipv4 address(`from`) and `port`.
/// 
/// This function actually only returns an error if the connection fails to be established. Otherwise it will loop forever.
fn receive(from: String, port: u16, config: Config) -> Result<(), Box<dyn Error>> {
    if !util::is_ipv4_address(&from) {
        return Err("Invalid address".into());
    }

    let listener = TcpListener::bind(format!("{}:{}", &from, port))?;
    println!("Listening on {}:{}", from, port);

    for stream in listener.incoming() {
        let mut stream = stream?;
        println!("Connection established: {:?}", stream);

        let mut buffer = Vec::new();
        stream.read_to_end(&mut buffer)?;
        let rv = bincode::deserialize::<Raven>(&buffer)?;

        match rv {
            Raven::Text { text } => println!("Received text: {}", text),
            Raven::File { name, content } => {
                let raven_arrivals = format!("{}/data", &config.raven_home);
                util::ensure_folder(&raven_arrivals)?;
                
                std::fs::write(format!("{}/{}", raven_arrivals, name), content)?;
                println!("Received file: {}", name);
            }
        }
    }

    Ok(())
}

/// Sends a message by a raven to another client.
/// The target client is specified by the `to` ipv4 address and `port`. The message is a `String`.
/// It will send only one message and finishes, the TCP protocol will take care of the rest.
/// If the target is offline, the connection will fail and the function will return an error.
fn send(to: &str, port: u16, message: String) -> Result<(), Box<dyn Error>> {
    if !util::is_ipv4_address(to) {
        return Err("Invalid address".into());
    }

    let mut stream = TcpStream::connect(format!("{}:{}", to, port))?;
    println!("Connected to {}:{}", to, port);

    let rv = Raven::Text { text: message };
    let encoded = bincode::serialize(&rv)?;

    stream.write_all(&encoded)?;
    println!("Message sent: {:?}", rv);

    Ok(())
}

/// Sends a file by a raven to another client.
/// The target client is specified by the `to` ipv4 address and `port`. The file is a `String` with the file path.
/// It will send only one file and finishes, the TCP protocol will take care of the rest.
/// If the target is offline, the connection will fail and the function will return an error.
/// If the file isn't found, the function will return an error.
fn send_file(to: &str, port: u16, file: String) -> Result<(), Box<dyn Error>> {
    if !util::is_ipv4_address(to) {
        return Err("Invalid address".into());
    }

    let mut stream = TcpStream::connect(format!("{}:{}", to, port))?;
    println!("Connected to {}:{}", to, port);

    let mut f = std::fs::File::open(&file)?;
    let mut content = Vec::new();
    f.read_to_end(&mut content)?;

    let rv = Raven::File {
        name: basename(&file).to_string(),
        content,
    };

    let encoded = bincode::serialize(&rv)?;

    stream.write_all(&encoded)?;
    println!("File sent: {:?}", rv);

    Ok(())
}