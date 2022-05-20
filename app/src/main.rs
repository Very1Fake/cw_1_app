use anyhow::Result;
use eframe::{run_native, NativeOptions};
use tokio::runtime::Builder;

use app::App;
use tracing::Level;
use tracing_subscriber::fmt;

mod app;
mod model;
mod utils;
mod view;

fn main() -> Result<()> {
    fmt()
        .compact()
        .with_max_level(if cfg!(debug_assertions) {
            Level::TRACE
        } else {
            Level::INFO
        })
        .init();

    let runtime = Builder::new_multi_thread()
        .worker_threads(2)
        .max_blocking_threads(4)
        .enable_all()
        .build()?;

    run_native(
        "CW App",
        NativeOptions::default(),
        Box::new(|cc| Box::new(App::new(cc, runtime))),
    )
}

// TODO: Secure authorization process
// TODO: Complex queries/forms

