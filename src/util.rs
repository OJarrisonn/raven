use std::{error::Error, fmt::{self, Display, Formatter}};

use regex::Regex;

#[derive(Debug)]
pub enum RavenError {
    FileNotFound,
    InvalidAddress(String)
}

impl Display for RavenError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            RavenError::FileNotFound => write!(f, "File not found"),
            RavenError::InvalidAddress(address) => write!(f, "Invalid address format: {}", address)
        }
    }
}

impl Error for RavenError {}

pub fn assert_ipv4_address(address: &str) -> bool {
    Regex::new("^(?:[0-9]{1,3}\\.){3}[0-9]{1,3}$").unwrap().is_match(address)
}