//! Responsible for the HELO and BYE protocol to stablish and finish a connection between clients.

use std::net::TcpStream;

use anyhow::Result;
use rsa::RsaPublicKey;

use crate::config::Config;

fn _helo(mut _stream: TcpStream, config: &Config) -> Result<(TcpStream, RsaPublicKey)> {
    let _pk = crate::auth::get_keypair(config)?.1;

    todo!("Implement the HELO protocol")
}