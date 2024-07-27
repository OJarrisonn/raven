use std::io::ErrorKind;

use anyhow::{Context, Result};
use interprocess::local_socket::{GenericFilePath, Name, ToFsName};
use serde::{Deserialize, Serialize};

use crate::util::{self, ensure_folder, LISTEN_DEFAULT_ADDRESS, LISTEN_DEFAULT_PORT, LOCAL_DEFAULT_PORT};

/// Describes the configuration of the raven client.
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    /// The path to the home folder of the raven client.
    #[serde(skip, default = "Config::raven_home")]
    pub raven_home: String,
    /// The receiver configuration.
    #[serde(default = "Receiver::default")]
    pub receiver: Receiver,
    #[serde(default = "Local::default")]
    pub local: Local,
}

/// Describes the configuration of the receiver.
#[derive(Debug, Serialize, Deserialize)]
pub struct Receiver {
    /// The ipv4 address where the receiver will listen.
    #[serde(default = "util::listen_default_address")]
    pub address: String,
    /// The port where the receiver will listen.
    #[serde(default = "util::listen_default_port")]
    pub port: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Local {
    /// The ipv4 address where the receiver will listen.
    #[serde(default = "util::listen_default_address")]
    pub address: String,
    /// The port where the receiver will listen.
    #[serde(default = "util::listen_default_port")]
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
            Ok(mut path) => {
                if path.ends_with('/') {
                    path.pop();
                }
                path
            }
            Err(_) => {
                let path = homedir::my_home().unwrap().unwrap();

                format!("{}/.raven", path.to_str().unwrap())
            }
        }
    }

    pub fn raven_sock_name(&self) -> Result<Name> {
        format!("{}/raven.sock", self.raven_home).to_fs_name::<GenericFilePath>().map_err(Into::into)
    }

    /// Loads the configuration from the raven home folder in config.toml.
    pub fn load() -> Result<Self> {
        let config_path = format!("{}/config.toml", Self::raven_home());

        match std::fs::read_to_string(&config_path) {
            Ok(config) => Ok(toml::from_str(&config).context(format!(
                "Failed to deserialize the config file from {}",
                &config_path
            ))?),
            Err(e) => {
                if ErrorKind::NotFound == e.kind() {
                    let config = Self::new();

                    config
                        .save()
                        .context("Failed to save the just created config file")?;

                    Ok(config)
                } else {
                    Err(e).context(format!(
                        "Failed to read the config file from {}",
                        &config_path
                    ))
                }
            }
        }
    }

    /// Saves the configuration to the raven home folder in config.toml.
    pub fn save(&self) -> Result<()> {
        let config_path = format!("{}/config.toml", self.raven_home);
        let config = toml::to_string(self)?;

        ensure_folder(&self.raven_home)?;
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
            local: Default::default(),
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

impl Default for Local {
    fn default() -> Self {
        Self {
            address: LISTEN_DEFAULT_ADDRESS.into(),
            port: LOCAL_DEFAULT_PORT
        }
    }
}