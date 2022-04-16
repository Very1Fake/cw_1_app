use clap::{Args, Parser};

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
    #[clap(alias = "gen")]
    Generate {
        #[clap(flatten)]
        uri: DatabaseUri,
    },
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

#[derive(Args, Debug)]
pub struct DatabaseUri {
    /// Database uri (e.g. 'postgres://user:pass@host:port/db')
    #[clap(short = 'd', long = "uri", env = "DB_URI")]
    pub inner: String,
}
