use anyhow::Result;
use app::app;
use clap::Parser;
use cw_core::sqlx::{
    postgres::{PgConnectOptions, PgSslMode},
    PgPool,
};
use opt::Opt;

mod app;
mod opt;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let args = Opt::parse();

    // Connecting to database
    let pool = PgPool::connect_with(
        PgConnectOptions::new()
            .host(args.host.as_str())
            .port(args.port)
            .username(args.username.as_str())
            .password(args.password.as_str())
            .database(args.database.as_str())
            .ssl_mode(PgSslMode::Prefer),
    )
    .await?;

    app(args, pool).await?;

    Ok(())
}
