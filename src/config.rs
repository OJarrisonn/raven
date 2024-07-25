use serde_derive::{Deserialize, Serialize};

use crate::util::{LISTEN_DEFAULT_ADDRESS, LISTEN_DEFAULT_PORT};

/// Describes the configuration of the raven client.
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    /// The path to the home folder of the raven client.
    pub raven_home: String,
    /// The receiver configuration.
    pub receiver: Receiver,
}

/// Describes the configuration of the receiver.
#[derive(Debug, Serialize, Deserialize)]
pub struct Receiver {
    /// The ipv4 address where the receiver will listen.
    pub address: String,
    /// The port where the receiver will listen.
    pub port: u16,
}

impl Config {
    /// Creates a new `Config` with the default values.
    pub fn new() -> Self {
        Default::default()
    }

    pub fn raven_home() -> String {
        let raven_home = std::env::var("RAVEN_HOME");

        match raven_home {
            Ok(path) => path,
            Err(_) => {
                let path = homedir::my_home().unwrap().unwrap();

                format!("{}/.raven", path.to_str().unwrap())
            },
        }
    }

    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = format!("{}/config.toml", Self::raven_home());
        let config = std::fs::read_to_string(config_path)?;

        Ok(toml::from_str(&config)?)
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = format!("{}/config.toml", self.raven_home);
        let config = toml::to_string(self)?;

        std::fs::write(config_path, config)?;

        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            // TODO: add context for when HOME env var is not set
            raven_home: Self::raven_home(),
            receiver: Default::default(),
        }
    }
}

impl Default for Receiver {
    fn default() -> Self {
        Receiver {
            address: LISTEN_DEFAULT_ADDRESS.into(),
            port: LISTEN_DEFAULT_PORT,
        }
    }
}