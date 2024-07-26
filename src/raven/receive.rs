use std::{io::Read, net::TcpListener};

use anyhow::{bail, Context, Result};

use crate::{
    config::Config,
    raven::{mailbox::MailBox, Raven},
    util,
};

/// Opens the client for receiving messages from a raven
/// The receiver works in a loop, listening for incoming connections and printing the received message.
/// The receiver will listen on the provided ipv4 address(`from`) and `port`.
///
/// This function actually only returns an error if the connection fails to be established. Otherwise it will loop forever.
pub fn receive(from: String, port: u16, config: Config) -> Result<()> {
    if !util::is_ipv4_address(&from) {
        bail!("Invalid ipv4 address {}", from);
    }

    let listener = TcpListener::bind(format!("{}:{}", &from, port))?;
    println!("Listening on {}:{}", from, port);

    for stream in listener.incoming() {
        let mut stream = match stream {
            Ok(stream) => stream,
            Err(e) => {
                eprintln!("Failed to establish a connection with: {}", e);
                continue;
            }
        };

        let sender = stream
            .peer_addr()
            .map(|addr| addr.to_string())
            .unwrap_or("".into());

        println!("Connection established: {}", &sender);

        let mut buffer = Vec::new();
        if let Err(e) = stream.read_to_end(&mut buffer).context("Receiving raven") {
            eprintln!("Failed to read the message: {}", e);
            continue;
        }
        let rv = match bincode::deserialize::<Raven>(&buffer).context("Deserializing received raven") {
            Ok(rv) => rv,
            Err(e) => {
                eprintln!("Failed to deserialize the received raven: {}", e);
                continue;
            }
        };

        let mut mailbox = MailBox::open(&config).context("Opening the mailbox")?; // Opens the mailbox to save the received messages

        match rv {
            Raven::Text { text } => {
                mailbox.add_message(sender, chrono::Utc::now(), text);
                mailbox.save(&config)?;
            }
            Raven::File { name, content } => {
                // Gets the folder where the files will be stored and ensures that it exists
                let raven_arrivals = format!("{}/data", &config.raven_home);
                if let Err(e) = util::ensure_folder(&raven_arrivals)
                    .context("Failed to create the folder to store files")
                {
                    eprintln!("{}", e);
                    continue;
                }

                // Gets a non colliding filename
                let path = format!("{}/{}", raven_arrivals, name);
                let path = util::non_colliding_filename(&path);

                // Writes the file to the disk
                if let Err(e) = std::fs::write(&path, content).context("Saving the received file") {
                    eprintln!("Failed to write the file: {}", e);
                    continue;
                }

                mailbox.add_file(sender, chrono::Utc::now(), path);
                mailbox
                    .save(&config)
                    .context("Saving the mailbox with the new received data")?;
            }
        }
    }

    Ok(())
}
