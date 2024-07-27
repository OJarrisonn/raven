use std::{io::Read, net::TcpStream, sync::Arc};

use anyhow::{bail, Context, Result};

use rv_raven::{
    config::Config, 
    raven::{mailbox::MailBox, Raven},
    util,
};

/// Opens the client for receiving messages from a raven
/// The receiver works in a loop, listening for incoming connections and printing the received message.
/// The receiver will listen on the provided ipv4 address(`from`) and `port`.
///
/// This function actually only returns an error if the connection fails to be established. Otherwise it will loop forever.
pub fn receive(mut stream: TcpStream, config: Arc<Config>) -> Result<()> {
    let sender = stream
        .peer_addr()
        .map(|addr| addr.to_string())
        .unwrap_or("".into());

    println!("Connection established: {}", &sender);

    let mut buffer = Vec::new();
    if let Err(e) = stream.read_to_end(&mut buffer).context("Receiving raven") {
        bail!("Failed to read the message {}", e);
    }

    let rv = match bincode::deserialize::<Raven>(&buffer).context("Deserializing received raven") {
        Ok(rv) => rv,
        Err(e) => {
            bail!("Failed to deserialize the received raven: {}", e);
        }
    };

    match rv {
        Raven::Text { text } => message(&config, sender, text),
        Raven::File { name, content } => file(&config, sender, name, content)
    }
}

fn message(config: &Config, sender: String, text: String) -> Result<()> {
    let mut mailbox = MailBox::open(&config).context("Opening the mailbox")?; // Opens the mailbox to save the received messages
    mailbox.add_message(sender, chrono::Utc::now(), text);
    mailbox.save(&config)?;
    Ok(())
}

fn file(config: &Config, sender: String, name: String, content: Vec<u8>) -> Result<()> {
    // Gets the folder where the files will be stored and ensures that it exists
    let raven_arrivals = format!("{}/data", &config.raven_home);
    if let Err(e) = util::ensure_folder(&raven_arrivals).context("Failed to create the folder to store files") {
        bail!("{}", e);
    }

    // Gets a non colliding filename
    let path = format!("{}/{}", raven_arrivals, name);
    let path = util::non_colliding_filename(&path);

    // Writes the file to the disk
    if let Err(e) = std::fs::write(&path, content).context("Saving the received file") {
        bail!("Failed to write the file: {}", e);
    }

    let mut mailbox = MailBox::open(&config).context("Opening the mailbox")?; // Opens the mailbox to save the received messages
    mailbox.add_file(sender, chrono::Utc::now(), path);
    mailbox.save(&config)?;
    
    Ok(())
}