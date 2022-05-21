use std::{sync::Arc, time::Duration};

use anyhow::Result;

use cw_core::sqlx::{
    pool::PoolOptions,
    postgres::{PgConnectOptions, PgSslMode},
    Error, PgPool,
};

use crate::model::config::SslMode;

pub type Pool = Arc<PgPool>;

pub async fn open_pool(uri: String, ssl_mode: SslMode, bound: (u32, u32)) -> Result<PgPool, Error> {
    let options: PgConnectOptions = uri.parse()?;

    // Connecting to database
    let pool = PoolOptions::new()
        .min_connections(bound.0)
        .max_connections(bound.1)
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
