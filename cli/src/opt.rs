use clap::{ArgEnum, Parser};

/// Utility for fast operations with database
#[derive(Parser, PartialEq, Debug)]
#[clap(author, version, about)]
pub struct Opt {
    /// Database host (e.g. '127.0.0.0')
    #[clap(short, long, default_value = "localhost", env = "DB_HOST")]
    pub host: String,
    /// Database port (5432 by default)
    #[clap(long, default_value = "5432", env = "DB_PORT")]
    pub port: u16,
    /// Database username
    #[clap(short, long, env = "DB_USER")]
    pub username: String,
    /// User password
    #[clap(short, long, env = "DB_PASS")]
    pub password: String,
    /// Database name
    #[clap(short, long, default_value = "cw1_db", env = "DB_NAME")]
    pub database: String,
    /// Available operations
    #[clap(arg_enum)]
    pub op: Op,
}

#[derive(ArgEnum, PartialEq, Clone, Debug)]
pub enum Op {
    Create,
    Drop,
    Check,
}
