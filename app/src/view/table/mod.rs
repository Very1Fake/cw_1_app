use eframe::egui::{TextStyle, Ui};
use egui_extras::{TableBuilder, TableRow};

pub const ID_WIDTH: f32 = 40.0;
pub const UUID_WIDTH: f32 = 245.0;
pub const COUNTRY_WIDTH: f32 = 60.0;
pub const TIMESTAMP_WIDTH: f32 = 190.0;

pub struct Table;

impl Table {
    pub fn draw(
        ui: &mut Ui,
        columns: impl FnOnce(TableBuilder) -> TableBuilder,
        headers: &[&str],
        rows: (usize, impl FnMut(usize, TableRow)),
    ) {
        let header_height = TextStyle::Heading.resolve(ui.style()).size;
        let row_height = TextStyle::Body.resolve(ui.style()).size;

        let table = columns(TableBuilder::new(ui).striped(true).resizable(true));
        table
            .header(header_height, |mut header| {
                headers.iter().for_each(|&title| {
                    header.col(|ui| {
                        ui.heading(title);
                    });
                })
            })
            .body(|body| {
                body.rows(row_height, rows.0, rows.1);
            });
    }
}
