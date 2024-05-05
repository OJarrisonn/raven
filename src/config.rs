use std::{collections::HashSet, fs};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct RavenConfig {
    #[serde(skip_serializing)]
    pub config_file: String,
    pub feather: String,
    pub known_feathers: HashSet<Feather>,
    pub permissions: PermissionConfig,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct Feather {
    pub feather: String,
    pub host: String,
    pub alias: String,
}

#[derive(Serialize, Deserialize)]
pub struct PermissionConfig {
    pub known_max_bytes: usize,
    pub unknown_max_bytes: usize,
    pub unknown_files: bool
}

impl RavenConfig {
    pub fn load(config_file: String) -> Result<RavenConfig, Box<dyn std::error::Error>> {
        let config = fs::read_to_string(&config_file)?;

        let mut config: RavenConfig = toml::from_str(&config)?;
        config.config_file = config_file;

        Ok(config)
    }

    pub fn load_or_create(config_file: String) -> Result<RavenConfig, Box<dyn std::error::Error>> {
        match Self::load(config_file.clone()) {
            Ok(config) => Ok(config),
            Err(_) => {
                let mut config = RavenConfig::default();
                config.config_file = config_file;
                config.save()?;
                Ok(config)
            },
        }
    }

    pub fn reload(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let updated = Self::load(self.config_file.clone())?;
        *self = updated;
        Ok(())
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config = toml::to_string(self)?;

        fs::write(&self.config_file, config)?;

        Ok(())
    }
}

impl Default for RavenConfig {
    fn default() -> Self {
        Self { 
            config_file: Default::default(), 
            feather: Uuid::new_v4().into(), 
            known_feathers: Default::default(), 
            permissions: PermissionConfig {
                known_max_bytes: 0,
                unknown_max_bytes: 1024,
                unknown_files: false,
            } 
        }
    }
}