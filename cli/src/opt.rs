use clap::{Args, Parser};

/// Utility for fast operations with database
#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Opt {
    /// SubCommands
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Parser, Debug)]
pub enum Command {
    #[clap(alias = "db")]
    Database(DatabaseOpt),
    #[clap(alias = "gen")]
    Generate {},
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
}

#[derive(Args, Debug)]
pub struct DatabaseUri {
    /// Database uri (e.g. 'postgres://user:pass@host:port/db')
    #[clap(short = 'd', long = "uri", env = "DB_URI")]
    pub inner: String,
}
