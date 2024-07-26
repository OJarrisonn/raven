use std::{
    io::{Read, Write},
    net::TcpStream,
};

use anyhow::{bail, Context, Result};

use crate::{raven::Raven, util};

/// Sends a message by a raven to another client.
/// The target client is specified by the `to` ipv4 address and `port`. The message is a `String`.
/// It will send only one message and finishes, the TCP protocol will take care of the rest.
/// If the target is offline, the connection will fail and the function will return an error.
pub fn send(to: &str, port: u16, message: String) -> Result<()> {
    if !util::is_ipv4_address(to) {
        bail!("Invalid ipv4 address {}", to);
    }

    let mut stream = TcpStream::connect(format!("{}:{}", to, port)).context(format!(
        "Connecting to the target client at {}:{}",
        to, port
    ))?;
    println!("Connected to {}:{}", to, port);

    let rv = Raven::Text { text: message };
    let encoded = bincode::serialize(&rv).context("Serializing message")?;

    stream.write_all(&encoded)?;
    println!("Message sent: {:?}", rv);

    Ok(())
}

/// Sends a file by a raven to another client.
/// The target client is specified by the `to` ipv4 address and `port`. The file is a `String` with the file path.
/// It will send only one file and finishes, the TCP protocol will take care of the rest.
/// If the target is offline, the connection will fail and the function will return an error.
/// If the file isn't found, the function will return an error.
pub fn send_file(to: &str, port: u16, file: String) -> Result<()> {
    if !util::is_ipv4_address(to) {
        bail!("Invalid address");
    }

    let mut stream = TcpStream::connect(format!("{}:{}", to, port)).context(format!(
        "Connecting to the target client at {}:{}",
        to, port
    ))?;
    println!("Connected to {}:{}", to, port);

    let mut f = std::fs::File::open(&file).context(format!("Opening file {} to be sent", &file))?;
    let mut content = Vec::new();
    f.read_to_end(&mut content)
        .context(format!("Reading file {} to be sent", &file))?;

    let rv = Raven::File {
        name: util::basename(&file).to_string(),
        content,
    };

    let encoded = bincode::serialize(&rv).context("Serializing file")?;

    stream
        .write_all(&encoded)
        .context("Writing to TCP stream")?;
    println!("File sent: {:?}", rv);

    Ok(())
}
