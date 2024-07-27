use std::{io::{Read, Write}, net::TcpStream};

use anyhow::{bail, Context, Result};
use rv_raven::{config::Config, raven::SysRaven, util};

pub fn send(config: &Config, to: &str, port: u16, message: &str) -> Result<()> {
    if !util::is_ipv4_address(to) {
        bail!("Invalid ipv4 address {}", to);
    }

    let mut stream = TcpStream::connect(format!("{}:{}", &config.local.address, config.local.port)).context("Error connecting to the local socket")?;

    let message = SysRaven::Send { to: to.to_string(), port, message: message.to_string() };
    let encoded = bincode::serialize(&message).context("Error serializing message")?;

    stream.write_all(&encoded).context("Error sending message")?;

    let mut buffer = vec![];
    let response = stream.read_to_end(&mut buffer).context("Error reading response")?;
    if response == 0 {
        bail!("Error receiving response");
    }
    let response = bincode::deserialize::<SysRaven>(&buffer).context("Error deserializing response")?;
    
    match response {
        SysRaven::Ok => {
            println!("Message sent");
            Ok(())
        }
        SysRaven::Error { message } => {
            bail!("Error sending message: {}", message);
        }
        _ => {
            bail!("Unexpected response");
        }
    }
}