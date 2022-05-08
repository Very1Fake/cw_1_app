use std::{path::PathBuf, str::FromStr};

use clap::{Arg, ArgMatches, Command as Cmd};

fn db_uri() -> [Arg<'static>; 2] {
    [
        Arg::new("ssl_mode")
            .short('s')
            .long("sslmode")
            .value_name("SSL_MODE")
            .default_value("Require")
            .possible_values(&[
                "Disable",
                "Allow",
                "Prefer",
                "Require",
                "VerifyCa",
                "VerifyFull",
            ])
            .validator(|v| v.parse::<SslMode>())
            .env("DB_SSL")
            .help("Set SSL mode")
            .required(true),
        Arg::new("uri")
            .short('u')
            .long("uri")
            .value_name("DB_URI")
            .env("DB_URI")
            .help("Database uri (e.g. 'postgres://user:pass@host:port/db')")
            .required(true),
    ]
}

pub fn get_opt() -> Cmd<'static> {
    Cmd::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .arg_required_else_help(true)
        .propagate_version(true)
        .args(&[
            Arg::new("min_pool")
                .short('m')
                .long("min_pool")
                .value_name("MIN_POOL")
                .default_value("1")
                .env("DB_MIN_POOL")
                .help("Minimum connections in the connection pool"),
            Arg::new("max_pool")
                .short('M')
                .long("max_pool")
                .value_name("MAX_POOL")
                .default_value("16")
                .env("DB_MAX_POOL")
                .help("Maximum connections in the connection pool"),
        ])
        .subcommand(
            Cmd::new("database")
                .arg_required_else_help(true)
                .alias("db")
                .about("Subcommand for performing actions on the database")
                .args(db_uri())
                .subcommand(
                    Cmd::new("create").alias("c").about(
                        "Create all necessary tables/procedures/trigger for the application",
                    ),
                )
                .subcommand(Cmd::new("drop").alias("d").about("Drop everything"))
                .subcommand(
                    Cmd::new("check")
                        .alias("ch")
                        .about("Check database for tables/procedures/triggers existence")
                        .arg(
                            Arg::new("fix")
                                .short('f')
                                .long("fix")
                                .help("Fix problem in place"),
                        ),
                )
                .subcommand(
                    Cmd::new("truncate")
                        .alias("t")
                        .about("Truncate low-level tables (with cascade option)"),
                )
                .subcommand_required(true),
        )
        .subcommand(
            Cmd::new("generate")
                .arg_required_else_help(true)
                .alias("gen")
                .about("Subcommand for generating sample/random data for the database")
                .subcommand(
                    Cmd::new("dump")
                        .alias("d")
                        .about("Create json file with generated data")
                        .arg(
                            Arg::new("path")
                                .short('p')
                                .long("path")
                                .default_value("./cw1_generator.dump.json")
                                .validator(|v| v.parse::<PathBuf>())
                                .help("Path to dump file")
                                .required(true),
                        ),
                )
                .subcommand(
                    Cmd::new("push")
                        .alias("p")
                        .about("Upload generated data to database")
                        .args(db_uri()),
                )
                .subcommand_required(true),
        )
}

pub fn parse() -> Option<Opt> {
    let args = get_opt().get_matches();

    if let Some(sub) = args.subcommand() {
        Some(Opt {
            pool_size: PoolSize::parse(&args),
            command: match sub {
                ("database", sub_args) => Command::Database(DatabaseOpt {
                    db: DatabaseUri::parse(sub_args),
                    action: if let Some(sub) = sub_args.subcommand() {
                        match sub {
                            ("create", _) => DatabaseAction::Create,
                            ("drop", _) => DatabaseAction::Drop,
                            ("check", sub_args) => DatabaseAction::Check {
                                fix: sub_args.is_present("fix"),
                            },
                            ("truncate", _) => DatabaseAction::Truncate,
                            (sub, _) => unreachable!("database subcommand: '{sub}'"),
                        }
                    } else {
                        return None;
                    },
                }),
                ("generate", sub_args) => {
                    Command::Generate(if let Some(sub) = sub_args.subcommand() {
                        match sub {
                            ("dump", sub_args) => Generate::Dump {
                                path: sub_args.value_of_t("path").expect("unreachable at path"),
                            },
                            ("push", sub_args) => Generate::Push {
                                uri: DatabaseUri::parse(sub_args),
                            },
                            (sub, _) => unreachable!("generator subcommand: '{sub}'"),
                        }
                    } else {
                        return None;
                    })
                }
                (sub, _) => unreachable!("primary subcommand: '{sub}'"),
            },
        })
    } else {
        None
    }
}

/// Utility for fast operations with database
#[derive(Debug)]
pub struct Opt {
    pub pool_size: PoolSize,
    /// Subcommands
    pub command: Command,
}

/// Struct stores the number of min and max connections for a connection pool
#[derive(Clone, Copy, Debug)]
pub struct PoolSize(pub u32, pub u32);

impl PoolSize {
    pub fn parse(args: &ArgMatches) -> Self {
        Self(
            args.value_of_t("min_pool")
                .expect("unreachable at min_pool"),
            args.value_of_t("max_pool")
                .expect("unreachable at max_pool"),
        )
    }
}

#[derive(Debug)]
pub enum Command {
    Database(DatabaseOpt),
    Generate(Generate),
}

#[derive(Debug)]
pub struct DatabaseOpt {
    pub db: DatabaseUri,
    pub action: DatabaseAction,
}

#[derive(Debug)]
pub enum DatabaseAction {
    Create,
    Drop,
    Check {
        /// Fix problem in place
        fix: bool,
    },
    Truncate,
}

#[derive(Debug)]
pub enum Generate {
    Push {
        uri: DatabaseUri,
    },
    Dump {
        /// Path to dump file
        path: PathBuf,
    },
}

#[derive(Debug)]
pub struct DatabaseUri {
    /// SSL mode
    pub ssl_mode: SslMode,
    /// Database uri (e.g. 'postgres://user:pass@host:port/db')
    pub inner: String,
}

impl DatabaseUri {
    pub fn parse(args: &ArgMatches) -> Self {
        Self {
            ssl_mode: args
                .value_of_t("ssl_mode")
                .expect("unreachable at ssl mode"),
            inner: args.value_of_t("uri").expect("unreachable at uri"),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum SslMode {
    Disable,
    Allow,
    Prefer,
    Require,
    VerifyCa,
    VerifyFull,
}

impl FromStr for SslMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SslMode::*;

        Ok(match s.to_lowercase().as_str() {
            "disable" => Disable,
            "allow" => Allow,
            "prefer" => Prefer,
            "require" => Require,
            "verifyca" => VerifyCa,
            "verifyfull" => VerifyFull,
            _ => return Err(format!("Invalid value '{s}' for ssl mode")),
        })
    }
}
