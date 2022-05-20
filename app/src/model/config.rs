use std::{fs::File, io::{ErrorKind, self}};

use anyhow::{Result, bail, Error};
use serde::{Deserialize, Serialize};
use serde_json::{from_reader, to_writer};
use tracing::{debug, info};

pub const CONFIG_PATH: &str = "./.config.json";

#[derive(Deserialize, Serialize, PartialEq, Default, Debug)]
pub struct Config {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection: Option<Connection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<Account>,
}

impl Config {
    fn empty(&self) -> bool {
        self.connection.is_none() && self.account.is_none()
    }

    fn reader() -> Result<Option<File>, io::Error> {
        match File::open(CONFIG_PATH) {
            Ok(file) => Ok(Some(file)),
            Err(err) if matches!(err.kind(), ErrorKind::NotFound) => return Ok(None),
            Err(err) => Err(err),
        }
    }

    pub fn load() -> Result<Config> {
        let file = if let Some(file) = Self::reader()? {
            file
        } else {
            return Ok(Self::default())
        };

        debug!("Config file found");

        let config = from_reader(file)?;

        info!(config = ?config, "Config loaded");

        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        if !self.empty() {
            if &Self::load()? == self {
                return Ok(())
            }
        
            to_writer(File::options().write(true).create(true).open(CONFIG_PATH)?, self)?;
        }

        Ok(())
    }
    
}

#[derive(Deserialize, Serialize, PartialEq, Default, Debug)]
pub struct Connection {
    pub host: String,
    pub user: String,
    pub password: String,
    pub database: String,
}

#[derive(Deserialize, Serialize, PartialEq, Default, Debug)]
pub struct Account {
    pub login: String,
    pub password: String,
}
