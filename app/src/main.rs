use eframe::{run_native, NativeOptions};

use app::App;

mod app;
mod model;

#[tokio::main]
async fn main() {
    run_native(
        "CW App",
        NativeOptions::default(),
        Box::new(|cc| Box::new(App::new(cc))),
    )
}

// TODO: Role-based main screen prototype
// TODO: Async sql queries
// TODO: Secure authorization process
// TODO: Auto random dataset generation
// TODO: Complex queries/forms

// TODO: Log tab for admin role
// TODO: Tracing log
