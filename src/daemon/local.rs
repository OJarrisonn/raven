//! This is the module responsible for the backend-frontend integration in raven
//! 
//! `rv` commands delegate IO operations to `rvd` through a local socket.

use std::{io::{Read, Write}, net::TcpListener, sync::Arc};

use anyhow::Result;
use rv_raven::{config::Config, raven::SysRaven};

pub fn local(config: Arc<Config>) -> Result<()> {
    let listener = TcpListener::bind(format!("{}:{}", &config.local.address, config.local.port))?;
    println!("Local socket created");

    listener.incoming().for_each(|stream| {
        let mut stream = stream.unwrap();

        let mut buffer = vec![];
        
        match stream.read_to_end(&mut buffer) {
            Ok(_) => {
                println!("Received message");
            }
            Err(e) => {
                eprintln!("Error reading message: {}", e);
                return;
            }
        }

        let message = bincode::deserialize::<rv_raven::raven::SysRaven>(&buffer).unwrap();
        println!("{:?}", message);
        
        let ok = SysRaven::Ok;
        let encoded = bincode::serialize(&ok).unwrap();
        stream.write_all(&encoded).unwrap();
    });

    Ok(())
}