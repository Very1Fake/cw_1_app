use eframe::{run_native, NativeOptions};

use app::App;

mod app;
mod models;

fn main() {
    run_native(Box::new(App::new()), NativeOptions::default())
}
