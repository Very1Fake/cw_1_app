use std::{
    fs::File,
    io::{self, ErrorKind},
};

use anyhow::Result;
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
            Err(err) if matches!(err.kind(), ErrorKind::NotFound) => Ok(None),
            Err(err) => Err(err),
        }
    }

    pub fn load() -> Result<Config> {
        let file = if let Some(file) = Self::reader()? {
            file
        } else {
            return Ok(Self::default());
        };

        debug!("Config file found");

        let config = from_reader(file)?;

        info!(config = ?config, "Config loaded");

        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        if !self.empty() {
            if &Self::load()? == self {
                return Ok(());
            }

            to_writer(
                File::options().write(true).create(true).open(CONFIG_PATH)?,
                self,
            )?;
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
    #[serde(default)]
    pub ssl_mode: SslMode,
}

#[derive(Deserialize, Serialize, PartialEq, Clone, Copy, Debug)]
#[repr(u8)]
pub enum SslMode {
    Disable = 0,
    Allow,
    Prefer,
    Require,
    VerifyCa,
    VerifyFull,
}

impl SslMode {
    pub const ALL: &'static [Self] = &[
        Self::Disable,
        Self::Allow,
        Self::Prefer,
        Self::Require,
        Self::VerifyCa,
        Self::VerifyFull,
    ];

    pub fn as_str(&self) -> &str {
        match self {
            SslMode::Disable => "Disable",
            SslMode::Allow => "Allow",
            SslMode::Prefer => "Prefer",
            SslMode::Require => "Require",
            SslMode::VerifyCa => "VerifyCa",
            SslMode::VerifyFull => "VerifyFull",
        }
    }
}

impl Default for SslMode {
    fn default() -> Self {
        Self::Prefer
    }
}

#[derive(Deserialize, Serialize, PartialEq, Default, Debug)]
pub struct Account {
    pub login: String,
    pub password: String,
}
