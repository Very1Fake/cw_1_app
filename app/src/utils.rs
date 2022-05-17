use std::{sync::Arc, time::Duration};

use anyhow::Result;

use cw_core::sqlx::{
    pool::PoolOptions,
    postgres::{PgConnectOptions, PgSslMode},
    Error, PgPool,
};

pub type Pool = Arc<PgPool>;

#[derive(Clone, Copy, Debug)]
pub enum SslMode {
    Disable,
    Allow,
    Prefer,
    Require,
    VerifyCa,
    VerifyFull,
}

pub async fn open_pool(uri: String, ssl_mode: SslMode) -> Result<PgPool, Error> {
    let options: PgConnectOptions = uri.parse()?;

    // Connecting to database
    let pool = PoolOptions::new()
        .min_connections(1)
        .max_connections(4)
        .connect_timeout(Duration::from_secs(4))
        .connect_with(options.application_name("CW-CLI").ssl_mode(match ssl_mode {
            SslMode::Disable => PgSslMode::Disable,
            SslMode::Allow => PgSslMode::Allow,
            SslMode::Prefer => PgSslMode::Prefer,
            SslMode::Require => PgSslMode::Require,
            SslMode::VerifyCa => PgSslMode::VerifyCa,
            SslMode::VerifyFull => PgSslMode::VerifyFull,
        }))
        .await?;

    Ok(pool)
}

// pub async fn setup() -> Result<()> {

// }
