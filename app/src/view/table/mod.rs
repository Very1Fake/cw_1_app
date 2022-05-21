use std::collections::BTreeMap;

use cw_core::{
    generator::Config,
    tables::{Manufacturer, Person, Position, Service, Supplier},
};
use eframe::egui::{TextStyle, Ui};
use egui_extras::{Size, TableBuilder, TableRow};
use tokio::runtime::Runtime;

use crate::{model::request::Request, utils::Pool};

pub type WindowStorage = BTreeMap<TableWindow, (bool, WindowState)>;

pub const ID_WIDTH: f32 = 40.0;
pub const UUID_WIDTH: f32 = 245.0;
pub const COUNTRY_WIDTH: f32 = 60.0;
pub const TIMESTAMP_WIDTH: f32 = 190.0;
pub const BUTTON_WIDTH: f32 = 20.0;

pub struct Table;

impl Table {
    pub fn draw(
        ui: &mut Ui,
        columns: &[Size],
        headers: &[&str],
        rows: (usize, impl FnMut(usize, TableRow)),
    ) {
        let header_height = TextStyle::Heading.resolve(ui.style()).size;
        let row_height = TextStyle::Body.resolve(ui.style()).size + 2.0;

        let mut table = TableBuilder::new(ui).striped(true).resizable(true);

        for size in columns {
            table = table.column(*size);
        }

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

#[derive(Debug)]
pub enum TableData {
    People { data: Vec<Person> },
    Positions { data: Vec<Position> },
    Manufacturers { data: Vec<Manufacturer> },
    Services { data: Vec<Service> },
    Suppliers { data: Vec<Supplier> },
}

impl From<TableWindow> for TableData {
    fn from(tabs: TableWindow) -> Self {
        match tabs {
            TableWindow::People => Self::People {
                data: Config::default().gen_person(),
            },
            TableWindow::Positions => Self::Positions {
                data: Config::default().gen_positions(),
            },
            TableWindow::Manufacturers => Self::Manufacturers {
                data: Config::default().gen_manufacturer(),
            },
            TableWindow::Services => Self::Services {
                data: Config::default().gen_service(),
            },
            TableWindow::Suppliers => Self::Suppliers {
                data: Config::default().gen_supplier(),
            },
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub enum TableWindow {
    People,
    Positions,
    Manufacturers,
    Services,
    Suppliers,
}

impl TableWindow {
    pub const ALL: &'static [Self] = &[
        Self::People,
        Self::Positions,
        Self::Manufacturers,
        Self::Services,
        Self::Suppliers,
    ];

    pub fn all() -> WindowStorage {
        let map = BTreeMap::from_iter(
            Self::ALL
                .iter()
                .map(|window| (*window, (false, WindowState::None))),
        );

        map
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::People => "People",
            Self::Positions => "Positions",
            Self::Manufacturers => "Manufacturers",
            Self::Services => "Services",
            Self::Suppliers => "Suppliers",
        }
    }
}

pub enum WindowState {
    None,
    Error(String),
    Loading(Request<(), TableData>, TableWindow),
    Loaded(TableData),
}

impl WindowState {
    pub fn is_visible(&self) -> bool {
        !matches!(self, Self::None)
    }

    pub fn load(runtime: &Runtime, pool: Pool, window: TableWindow) -> Self {
        Self::Loading(
            Request::simple(runtime, move || async move {
                Ok(match window {
                    TableWindow::People => TableData::People {
                        data: Person::get_all().fetch_all(&*pool).await?,
                    },
                    TableWindow::Positions => TableData::Positions {
                        data: Position::get_all().fetch_all(&*pool).await?,
                    },
                    TableWindow::Manufacturers => TableData::Manufacturers {
                        data: Manufacturer::get_all().fetch_all(&*pool).await?,
                    },
                    TableWindow::Services => TableData::Services {
                        data: Service::get_all().fetch_all(&*pool).await?,
                    },
                    TableWindow::Suppliers => TableData::Suppliers {
                        data: Supplier::get_all().fetch_all(&*pool).await?,
                    },
                })
            }),
            window,
        )
    }
}
