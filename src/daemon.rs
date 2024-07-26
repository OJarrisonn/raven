use std::{sync::Arc, thread};

use anyhow::Result;
use rv_raven::{config::Config, raven::receive::receive};

fn main() -> Result<()> {
    let config = Arc::new(Config::load()?);
    
    let listener = std::net::TcpListener::bind(format!("{}:{}", config.receiver.address, config.receiver.port))?;
    println!("Listening on {}:{}", config.receiver.address, config.receiver.port);

    // Accept connections and process them, spawning a new thread for each one
    for stream in listener.incoming() {
        let stream = stream?;
        let config = Arc::clone(&config);

        thread::spawn(move || {
            if let Err(e) = receive(stream, config) {
                eprintln!("Error: {}", e);
            }
        });
    }

    Ok(())
}