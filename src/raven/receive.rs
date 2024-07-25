use std::{error::Error, io::Read, net::TcpListener};

use crate::{config::Config, raven::Raven, util};

/// Opens the client for receiving messages from a raven
/// The receiver works in a loop, listening for incoming connections and printing the received message.
/// The receiver will listen on the provided ipv4 address(`from`) and `port`.
///
/// This function actually only returns an error if the connection fails to be established. Otherwise it will loop forever.
pub(crate) fn receive(from: String, port: u16, config: Config) -> Result<(), Box<dyn Error>> {
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
                // Gets the folder where the files will be stored and ensures that it exists
                let raven_arrivals = format!("{}/data", &config.raven_home);
                util::ensure_folder(&raven_arrivals)?;
                
                // Gets a non colliding filename
                let path = format!("{}/{}", raven_arrivals, name);
                let path = util::non_colliding_filename(&path);

                // Writes the file to the disk
                std::fs::write(&path, content)?;
                println!("Received file: {}", path);
            }
        }
    }

    Ok(())
}
