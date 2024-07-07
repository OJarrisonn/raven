use std::{collections::HashSet, fs, io};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::consts::LISTEN_DEFAULT_PORT;

#[derive(Serialize, Deserialize)]
pub struct RavenConfig {
    #[serde(skip)]
    pub config_file: String,
    pub feather: Feather,
    pub known_feathers: HashSet<Feather>,
    pub known_max_bytes: usize,
    pub unknown_max_bytes: usize,
    pub unknown_files: bool
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct Feather {
    pub id: String,
    pub host: String,
    pub port: u16,
    pub alias: String,
}

impl RavenConfig {
    fn load(config_file: String) -> Result<RavenConfig, Box<dyn std::error::Error>> {
        let config = fs::read_to_string(&config_file)?;

        let mut config: RavenConfig = toml::from_str(&config)?;
        config.config_file = config_file;

        Ok(config)
    }

    pub fn load_or_create(config_file: String) -> Result<RavenConfig, Box<dyn std::error::Error>> {
        match Self::load(config_file.clone()) {
            Ok(config) => Ok(config),
            Err(err) => {
                if err.is::<io::Error>() {
                    let mut config = RavenConfig::default();
                    config.config_file = config_file;
                    config.save()?;
                    Ok(config)
                } else {
                    Err(err)
                }
            },
        }
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config = toml::to_string(self)?;

        fs::write(&self.config_file, config)?;

        Ok(())
    }

    pub fn add_known_feather(&mut self, feather: Feather) {
        self.known_feathers.insert(feather);
    }
}

impl Default for RavenConfig {
    fn default() -> Self {
        Self { 
            config_file: Default::default(), 
            feather: Feather { id: Uuid::new_v4().into(), host: Default::default(), port: LISTEN_DEFAULT_PORT, alias: "@localhost".into() }, 
            known_feathers: Default::default(), 
            known_max_bytes: 0,
            unknown_max_bytes: 1024,
            unknown_files: false,
        }
    }
}