use anyhow::Result;
use app::app;
use opt::parse;

mod app;
mod opt;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let args = parse().unwrap();
    app(args).await?;
    Ok(())
}

// TODO: Move opt.rs to builder API
