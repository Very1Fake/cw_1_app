use std::path::PathBuf;

use clap::{ArgEnum, Args, Parser};

/// Utility for fast operations with database
#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Opt {
    // Pool options
    #[clap(flatten)]
    pub pool_opts: PoolOpts,
    /// SubCommands
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Args, Debug)]
pub struct PoolOpts {
    // Set the minimum number of connections in the pool
    #[clap(short = 'm', long, default_value = "1", env = "DB_MIN_CONNS")]
    pub min_conns: u32,
    // Set the maximum number of connections in the pool
    #[clap(short = 'M', long, default_value = "16", env = "DB_MAX_CONNS")]
    pub max_conns: u32,
}

#[derive(Parser, Debug)]
pub enum Command {
    #[clap(alias = "db")]
    Database(DatabaseOpt),
    #[clap(subcommand, alias = "gen")]
    Generate(Generate),
}

#[derive(Parser, Debug)]
pub struct DatabaseOpt {
    #[clap(flatten)]
    pub db: DatabaseUri,
    #[clap(subcommand)]
    pub command: Database,
}

#[derive(Parser, Debug)]
pub enum Database {
    #[clap(alias = "c")]
    Create,
    #[clap(alias = "d")]
    Drop,
    #[clap(alias = "ch")]
    Check {
        /// Fix problem in place
        #[clap(short, long)]
        fix: bool,
    },
    #[clap(alias = "t")]
    Truncate,
}

#[derive(Parser, Debug)]
pub enum Generate {
    #[clap(alias = "p")]
    Push {
        #[clap(flatten)]
        uri: DatabaseUri,
    },
    #[clap(alias = "d")]
    Dump {
        /// Path to dump file
        #[clap(short, long, default_value = "./cw1_generator.dump.json")]
        path: PathBuf,
    },
}

#[derive(Args, Debug)]
pub struct DatabaseUri {
    /// SSL mode
    #[clap(
        arg_enum,
        short = 's',
        long = "sslmode",
        default_value = "Require",
        env = "DB_SSL"
    )]
    pub ssl_mode: SslMode,
    /// Database uri (e.g. 'postgres://user:pass@host:port/db')
    #[clap(short = 'd', long = "uri", env = "DB_URI")]
    pub inner: String,
}

#[derive(ArgEnum, Clone, Copy, Debug)]
#[clap(rename_all = "PascalCase")]
pub enum SslMode {
    Disable,
    Allow,
    Prefer,
    Require,
    VerifyCa,
    VerifyFull,
}
