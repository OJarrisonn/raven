//! This module is responsible for the remote side of the application, connecting to remote `rvd` instances using network.

use std::{net::TcpListener, thread, sync::Arc};

use anyhow::Result;
use rv_raven::config::Config;

mod receive;

pub fn remote(config: Arc<Config>) -> Result<()> {
    let listener = TcpListener::bind(format!("{}:{}", config.receiver.address, config.receiver.port))?;
    println!("Listening on {}:{}", config.receiver.address, config.receiver.port);

    // Accept connections and process them, spawning a new thread for each one
    for stream in listener.incoming() {
        let stream = if let Err(e) = stream {
            eprintln!("Failed to establish a connection: {}", e);
            continue;
        } else {
            stream.unwrap()
        };

        let config = Arc::clone(&config);

        thread::spawn(move || {
            if let Err(e) = receive::receive(stream, config) {
                eprintln!("Error: {}", e);
            }
        });
    }

    Ok(())
}