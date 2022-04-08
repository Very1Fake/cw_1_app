use anyhow::Result;
use app::app;
use clap::Parser;
use opt::Opt;

mod app;
mod opt;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let args = Opt::parse();
    app(args).await?;
    Ok(())
}
