use std::{collections::BTreeMap, sync::Arc};

use cw_core::{
    generator::Config,
    tables::{Manufacturer, Person, Position, Service, Supplier},
};
use eframe::egui::{Context, TopBottomPanel, Window};
use egui_extras::Size;
use tokio::runtime::Runtime;

use crate::{
    model::{
        request::{Request, RequestStatus},
        user::User,
    },
    utils::Pool,
};

use super::table::{Table, COUNTRY_WIDTH, ID_WIDTH, TIMESTAMP_WIDTH, UUID_WIDTH};

pub type WindowStorage = BTreeMap<TableWindow, (bool, WindowState)>;

pub struct MainView {
    user: User,
    windows: WindowStorage,
}

impl MainView {
    pub fn new(user: User) -> Self {
        Self {
            user,
            windows: TableWindow::all(),
        }
    }

    pub fn update(&mut self, ctx: &Context, runtime: &Runtime, pool: Pool) {
        TopBottomPanel::top("main_tabs").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                for (window, (open, state)) in &mut self.windows {
                    if ui.selectable_label(*open, window.as_str()).clicked() {
                        if *open {
                            *open = false;
                            *state = WindowState::None;
                        } else {
                            *open = true;
                            *state = WindowState::load(runtime, Arc::clone(&pool), *window);
                        }
                    }
                }
            })
        });

        self.windows
            .iter_mut()
            .map(|(window, (open, state))| {
                if *open != state.is_visible() && !*open {
                    *state = WindowState::None;
                }
                (window, (open, state))
            })
            .for_each(|(window, (open, state))| {
                Window::new(window.as_str())
                    .open(open)
                    .resizable(true)
                    .scroll2([true; 2])
                    .show(ctx, |ui| match state {
                        WindowState::Loaded(window_data) => {
                            match window_data {
                                TableData::People { data } => Table::draw(
                                    ui,
                                    &[
                                        Size::exact(ID_WIDTH),
                                        Size::exact(UUID_WIDTH),
                                        Size::initial(120.0),
                                        Size::initial(120.0),
                                        Size::initial(120.0),
                                        Size::initial(120.0),
                                        Size::initial(120.0),
                                        Size::initial(TIMESTAMP_WIDTH),
                                        Size::initial(TIMESTAMP_WIDTH),
                                    ],
                                    &[
                                        "ID",
                                        "UUID",
                                        "First Name",
                                        "Middle Name",
                                        "Last Name",
                                        "Email",
                                        "Phone",
                                        "Updated",
                                        "Created",
                                    ],
                                    (data.len(), |index, mut row| match data.get(index) {
                                        Some(person) => {
                                            row.col(|ui| {
                                                ui.label(index.to_string());
                                            });
                                            row.col(|ui| {
                                                ui.label(format!("{}", person.uuid));
                                            });
                                            row.col(|ui| {
                                                ui.label(person.first_name.clone());
                                            });
                                            row.col(|ui| {
                                                ui.label(if person.middle_name.is_some() {
                                                    person.middle_name.clone().unwrap()
                                                } else {
                                                    String::new()
                                                });
                                            });
                                            row.col(|ui| {
                                                ui.label(person.last_name.clone());
                                            });
                                            row.col(|ui| {
                                                ui.label(person.email.clone());
                                            });
                                            row.col(|ui| {
                                                ui.label(person.phone.clone());
                                            });
                                            row.col(|ui| {
                                                ui.label(format!("{}", person.meta.updated));
                                            });
                                            row.col(|ui| {
                                                ui.label(format!("{}", person.meta.created));
                                            });
                                        }
                                        None => {
                                            row.col(|ui| {
                                                ui.label("Error while indexing");
                                            });
                                        }
                                    }),
                                ),
                                TableData::Positions { data } => Table::draw(
                                    ui,
                                    &[
                                        Size::exact(UUID_WIDTH),
                                        Size::initial(120.0),
                                        Size::initial(120.0),
                                        Size::initial(80.0),
                                        Size::exact(TIMESTAMP_WIDTH),
                                        Size::exact(TIMESTAMP_WIDTH),
                                    ],
                                    &["UUID", "Name", "Details", "Salary", "Updated", "Created"],
                                    (data.len(), |index, mut row| match data.get(index) {
                                        Some(position) => {
                                            row.col(|ui| {
                                                ui.label(format!("{}", position.uuid));
                                            });
                                            row.col(|ui| {
                                                ui.label(position.name.clone());
                                            });
                                            row.col(|ui| {
                                                ui.label(if position.details.is_some() {
                                                    position.details.clone().unwrap()
                                                } else {
                                                    String::new()
                                                });
                                            });
                                            row.col(|ui| {
                                                ui.label(format!(
                                                    "{}₽.",
                                                    position.salary.to_bigdecimal(2)
                                                ));
                                            });
                                            row.col(|ui| {
                                                ui.label(format!("{}", position.meta.updated));
                                            });
                                            row.col(|ui| {
                                                ui.label(format!("{}", position.meta.created));
                                            });
                                        }
                                        None => {
                                            row.col(|ui| {
                                                ui.label("Error while indexing");
                                            });
                                        }
                                    }),
                                ),
                                TableData::Manufacturers { data } => Table::draw(
                                    ui,
                                    &[
                                        Size::exact(UUID_WIDTH),
                                        Size::initial(120.0),
                                        Size::exact(50.0),
                                    ],
                                    &["UUID", "Name", "Country"],
                                    (data.len(), |index, mut row| match data.get(index) {
                                        Some(manufacturer) => {
                                            row.col(|ui| {
                                                ui.label(format!("{}", manufacturer.uuid));
                                            });
                                            row.col(|ui| {
                                                ui.label(manufacturer.name.clone());
                                            });
                                            row.col(|ui| {
                                                ui.label(manufacturer.country.clone());
                                            });
                                        }
                                        None => {
                                            row.col(|ui| {
                                                ui.label("Error while indexing");
                                            });
                                        }
                                    }),
                                ),
                                TableData::Services { data } => Table::draw(
                                    ui,
                                    &[
                                        Size::exact(UUID_WIDTH),
                                        Size::initial(120.0),
                                        Size::initial(120.0),
                                        Size::exact(TIMESTAMP_WIDTH),
                                        Size::exact(TIMESTAMP_WIDTH),
                                    ],
                                    &["UUID", "Name", "Description", "Updated", "Created"],
                                    (data.len(), |index, mut row| match data.get(index) {
                                        Some(service) => {
                                            row.col(|ui| {
                                                ui.label(format!("{}", service.uuid));
                                            });
                                            row.col(|ui| {
                                                ui.label(service.name.clone());
                                            });
                                            row.col(|ui| {
                                                ui.label(if service.description.is_some() {
                                                    service.description.clone().unwrap()
                                                } else {
                                                    String::new()
                                                });
                                            });
                                            row.col(|ui| {
                                                ui.label(format!("{}", service.meta.created));
                                            });
                                            row.col(|ui| {
                                                ui.label(format!("{}", service.meta.created));
                                            });
                                        }
                                        None => {
                                            row.col(|ui| {
                                                ui.label("Error while indexing");
                                            });
                                        }
                                    }),
                                ),
                                TableData::Suppliers { data } => Table::draw(
                                    ui,
                                    &[
                                        Size::exact(UUID_WIDTH),
                                        Size::initial(120.0),
                                        Size::exact(235.0),
                                        Size::initial(120.0),
                                        Size::initial(210.0),
                                        Size::exact(COUNTRY_WIDTH),
                                    ],
                                    &["UUID", "Name", "IBAN", "Swift", "Address", "Country"],
                                    (data.len(), |index, mut row| match data.get(index) {
                                        Some(supplier) => {
                                            row.col(|ui| {
                                                ui.label(format!("{}", supplier.uuid));
                                            });
                                            row.col(|ui| {
                                                ui.label(supplier.name.clone());
                                            });
                                            row.col(|ui| {
                                                ui.label(supplier.iban.clone());
                                            });
                                            row.col(|ui| {
                                                ui.label(supplier.swift.clone());
                                            });
                                            row.col(|ui| {
                                                ui.label(supplier.address.clone());
                                            });
                                            row.col(|ui| {
                                                ui.label(supplier.country.clone());
                                            });
                                        }
                                        None => {
                                            row.col(|ui| {
                                                ui.label("Error while indexing");
                                            });
                                        }
                                    }),
                                ),
                            };
                        }
                        WindowState::Loading(request, window) => {
                            match request.peek(runtime).status.take() {
                                RequestStatus::Finished(result) => {
                                    *state = match result {
                                        Ok(window_data) => WindowState::Loaded(window_data),
                                        Err(err) => WindowState::Error(format!("{err}")),
                                    }
                                }
                                _ => {
                                    ui.vertical_centered(|ui| {
                                        ui.spinner();
                                        ui.add_space(8.0);
                                        ui.heading(format!(
                                            "Loading \"{}\" table",
                                            window.as_str()
                                        ));
                                    });
                                }
                            }
                        }
                        WindowState::Error(msg) => {
                            ui.vertical_centered(|ui| {
                                ui.collapsing("Error occurred while loading table", |ui| {
                                    ui.label(msg.as_str());
                                })
                            });
                        }
                        WindowState::None => {
                            ui.label(format!(
                                "Hello: {} {}",
                                self.user.person.first_name, self.user.person.last_name
                            ));
                        }
                    });
            });
    }
}

// -------------------------------------------------------------------------------------------------

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
