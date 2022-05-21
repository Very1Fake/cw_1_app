use std::sync::Arc;

use cw_core::{
    generator::Config,
    tables::{Manufacturer, Person, Position, Service, Supplier},
};
use eframe::egui::{CentralPanel, Context, TopBottomPanel};
use egui_extras::Size;
use tokio::runtime::Runtime;
use tracing::warn;

use crate::{
    model::{
        request::{Request, RequestStatus},
        user::User,
    },
    utils::Pool,
};

use super::table::{Table, COUNTRY_WIDTH, ID_WIDTH, TIMESTAMP_WIDTH, UUID_WIDTH};

pub struct MainView {
    user: User,
    tab: TabHandler,
}

impl MainView {
    pub fn new(user: User) -> Self {
        warn!("Initialized main!");
        Self {
            user,
            tab: TabHandler::None,
        }
    }

    pub fn update(&mut self, ctx: &Context, runtime: &Runtime, pool: Pool) {
        TopBottomPanel::top("main_tabs").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                for tab in Tabs::ALL {
                    if ui
                        .selectable_label(
                            if let TabHandler::Loaded(current_tab) = &self.tab {
                                current_tab.as_tabs() == *tab
                            } else {
                                false
                            },
                            tab.as_str(),
                        )
                        .clicked()
                    {
                        self.tab = TabHandler::load(runtime, Arc::clone(&pool), *tab)
                    }
                }
            })
        });

        match &mut self.tab {
            TabHandler::Loaded(tab) => {
                CentralPanel::default().show(ctx, |ui| {
                    match tab {
                        Tab::People { data } => Table::draw(
                            ui,
                            |builder| {
                                builder
                                    .column(Size::exact(ID_WIDTH))
                                    .column(Size::exact(UUID_WIDTH))
                                    .column(Size::initial(120.0))
                                    .column(Size::initial(120.0))
                                    .column(Size::initial(120.0))
                                    .column(Size::initial(120.0))
                                    .column(Size::initial(120.0))
                                    .column(Size::initial(TIMESTAMP_WIDTH))
                                    .column(Size::initial(TIMESTAMP_WIDTH))
                            },
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
                        Tab::Positions { data } => Table::draw(
                            ui,
                            |builder| {
                                builder
                                    .column(Size::exact(UUID_WIDTH))
                                    .column(Size::initial(120.0))
                                    .column(Size::initial(120.0))
                                    .column(Size::initial(80.0))
                                    .column(Size::exact(TIMESTAMP_WIDTH))
                                    .column(Size::exact(TIMESTAMP_WIDTH))
                            },
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
                                        ui.label(format!("{}₽.", position.salary.to_bigdecimal(2)));
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
                        Tab::Manufacturers { data } => Table::draw(
                            ui,
                            |builder| {
                                builder
                                    .column(Size::exact(UUID_WIDTH))
                                    .column(Size::initial(120.0))
                                    .column(Size::exact(50.0))
                            },
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
                        Tab::Services { data } => Table::draw(
                            ui,
                            |builder| {
                                builder
                                    .column(Size::exact(UUID_WIDTH))
                                    .column(Size::initial(120.0))
                                    .column(Size::initial(120.0))
                                    .column(Size::exact(TIMESTAMP_WIDTH))
                                    .column(Size::exact(TIMESTAMP_WIDTH))
                            },
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
                        Tab::Suppliers { data } => Table::draw(
                            ui,
                            |builder| {
                                builder
                                    .column(Size::exact(UUID_WIDTH))
                                    .column(Size::initial(120.0))
                                    .column(Size::exact(235.0))
                                    .column(Size::initial(120.0))
                                    .column(Size::initial(210.0))
                                    .column(Size::exact(COUNTRY_WIDTH))
                            },
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
                });
            }
            TabHandler::Loading(request, tab) => match request.peek(runtime).status.take() {
                RequestStatus::Finished(result) => {
                    self.tab = match result {
                        Ok(tab) => TabHandler::Loaded(tab),
                        Err(err) => TabHandler::Error(format!("{err}")),
                    }
                }
                _ => {
                    CentralPanel::default().show(ctx, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.spinner();
                            ui.add_space(8.0);
                            ui.heading(format!("Loading \"{}\" table", tab.as_str()));
                        })
                    });
                }
            },
            TabHandler::Error(msg) => {
                CentralPanel::default().show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.collapsing("Error occurred while loading table", |ui| {
                            ui.label(msg.as_str());
                        })
                    })
                });
            }
            TabHandler::None => {
                CentralPanel::default().show(ctx, |ui| {
                    ui.label(format!(
                        "Hello: {} {}",
                        self.user.person.first_name, self.user.person.last_name
                    ));
                });
            }
        }
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub enum Tab {
    People { data: Vec<Person> },
    Positions { data: Vec<Position> },
    Manufacturers { data: Vec<Manufacturer> },
    Services { data: Vec<Service> },
    Suppliers { data: Vec<Supplier> },
}

impl Tab {
    pub fn as_tabs(&self) -> Tabs {
        match self {
            Self::People { .. } => Tabs::People,
            Self::Positions { .. } => Tabs::Positions,
            Self::Manufacturers { .. } => Tabs::Manufacturers,
            Self::Services { .. } => Tabs::Services,
            Self::Suppliers { .. } => Tabs::Suppliers,
        }
    }
}

impl From<Tabs> for Tab {
    fn from(tabs: Tabs) -> Self {
        match tabs {
            Tabs::People => Self::People {
                data: Config::default().gen_person(),
            },
            Tabs::Positions => Self::Positions {
                data: Config::default().gen_positions(),
            },
            Tabs::Manufacturers => Self::Manufacturers {
                data: Config::default().gen_manufacturer(),
            },
            Tabs::Services => Self::Services {
                data: Config::default().gen_service(),
            },
            Tabs::Suppliers => Self::Suppliers {
                data: Config::default().gen_supplier(),
            },
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Tabs {
    People,
    Positions,
    Manufacturers,
    Services,
    Suppliers,
}

impl Tabs {
    pub const ALL: &'static [Self] = &[
        Self::People,
        Self::Positions,
        Self::Manufacturers,
        Self::Services,
        Self::Suppliers,
    ];

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

pub enum TabHandler {
    None,
    Error(String),
    Loading(Request<(), Tab>, Tabs),
    Loaded(Tab),
}

impl TabHandler {
    pub fn load(runtime: &Runtime, pool: Pool, tab: Tabs) -> Self {
        Self::Loading(
            Request::simple(runtime, move || async move {
                Ok(match tab {
                    Tabs::People => Tab::People {
                        data: Person::get_all().fetch_all(&*pool).await?,
                    },
                    Tabs::Positions => Tab::Positions {
                        data: Position::get_all().fetch_all(&*pool).await?,
                    },
                    Tabs::Manufacturers => Tab::Manufacturers {
                        data: Manufacturer::get_all().fetch_all(&*pool).await?,
                    },
                    Tabs::Services => Tab::Services {
                        data: Service::get_all().fetch_all(&*pool).await?,
                    },
                    Tabs::Suppliers => Tab::Suppliers {
                        data: Supplier::get_all().fetch_all(&*pool).await?,
                    },
                })
            }),
            tab,
        )
    }
}
