use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Database {
    #[serde(default = "default_addr")]
    pub addr: String,
    #[serde(default = "default_port")]
    pub port: String,
    pub user: String,
    pub pass: String,
    pub name: String,
}

impl fmt::Display for Database {
    /// Actually displays a connection string from `Database` fields
    ///
    /// # Note
    ///
    /// Only PostgreSQL is supported
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "postgres://{}:{}@{}:{}/{}",
            self.user, self.pass, self.addr, self.port, self.name
        )
    }
}

fn default_addr() -> String {
    String::from("localhost")
}

fn default_port() -> String {
    String::from("5432")
}
