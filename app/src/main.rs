use eframe::{run_native, NativeOptions};

use app::App;

mod app;
mod model;

#[tokio::main]
async fn main() {
    run_native(Box::new(App::new()), NativeOptions::default())
}

// TODO: Role-based main screen prototype
// TODO: Async sql queries
// TODO: Secure authorization process
// TODO: Auto random dataset generation
// TODO: Complex queries/forms

// TODO: Log tab for admin role
// TODO: Tracing log
