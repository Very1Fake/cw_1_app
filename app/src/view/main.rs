// use std::mem::discriminant;

use cw_core::{
    generator::Config,
    tables::{Manufacturer, Person, Position, Service, Supplier},
};
use eframe::egui::{CentralPanel, Context, TopBottomPanel};
use egui_extras::{Size, TableBuilder};
use tokio::runtime::Runtime;

use crate::{
    model::{request::Request, user::User},
    utils::Pool,
};

#[derive(Debug)]
pub enum Tab {
    People { data: Vec<Person> },
    Positions { data: Vec<Position> },
    Manufacturers { data: Vec<Manufacturer> },
    Services { data: Vec<Service> },
    Suppliers { data: Vec<Supplier> },
}

impl Tab {
    // pub fn equal(lhs: &Self, rhs: &Self) -> bool {
    //     discriminant(lhs) == discriminant(rhs)
    // }

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

    // pub fn equal(lhs: &Self, rhs: &Self) -> bool {
    //     discriminant(lhs) == discriminant(rhs)
    // }
}

// -------------------------------------------------------------------------------------------------

pub struct MainView {
    user: User,
    tab: TabHandler,
}

impl MainView {
    pub fn new(user: User) -> Self {
        Self {
            user,
            tab: TabHandler::None,
        }
    }

    pub fn update(&mut self, ctx: &Context, _runtime: &Runtime, _pool: Pool) {
        TopBottomPanel::top("main_tabs").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                for tab in Tabs::ALL {
                    if ui
                        .selectable_label(
                            if let TabHandler::Loaded(current_tab) = &self.tab {
                                if current_tab.as_tabs() == *tab {
                                    true
                                } else {
                                    false
                                }
                            } else {
                                false
                            },
                            tab.as_str(),
                        )
                        .clicked()
                    {
                        self.tab = TabHandler::Loaded((*tab).into())
                    }
                }
            })
        });

        match &self.tab {
            TabHandler::Loaded(tab) => {
                CentralPanel::default().show(ctx, |ui| {
                    match tab {
                        Tab::People { data } => TableBuilder::new(ui)
                            .striped(true)
                            .resizable(true)
                            .column(Size::exact(240.0))
                            .column(Size::initial(120.0))
                            .column(Size::initial(120.0))
                            .column(Size::initial(120.0))
                            .column(Size::initial(120.0))
                            .column(Size::initial(120.0))
                            .column(Size::initial(210.0))
                            .column(Size::initial(210.0))
                            .header(20.0, |mut header| {
                                header.col(|ui| {
                                    ui.heading("UUID");
                                });
                                header.col(|ui| {
                                    ui.heading("First Name");
                                });
                                header.col(|ui| {
                                    ui.heading("Middle Name");
                                });
                                header.col(|ui| {
                                    ui.heading("Last Name");
                                });
                                header.col(|ui| {
                                    ui.heading("Email");
                                });
                                header.col(|ui| {
                                    ui.heading("Phone");
                                });
                                header.col(|ui| {
                                    ui.heading("Updated");
                                });
                                header.col(|ui| {
                                    ui.heading("Created");
                                });
                            })
                            .body(|mut body| {
                                for person in data {
                                    body.row(24.0, |mut row| {
                                        row.col(|ui| {
                                            ui.label(format!("{}", person.uuid));
                                        });
                                        row.col(|ui| {
                                            ui.label(person.first_name.clone());
                                        });
                                        row.col(|ui| {
                                            ui.label(
                                                if let Some(name) = person.middle_name.clone() {
                                                    name
                                                } else {
                                                    String::new()
                                                },
                                            );
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
                                    })
                                }
                            }),
                        Tab::Positions { data } => TableBuilder::new(ui)
                            .striped(true)
                            .resizable(true)
                            .column(Size::exact(240.0))
                            .column(Size::initial(120.0))
                            .column(Size::initial(120.0))
                            .column(Size::initial(80.0))
                            .column(Size::initial(210.0))
                            .column(Size::initial(210.0))
                            .header(20.0, |mut header| {
                                header.col(|ui| {
                                    ui.heading("UUID");
                                });
                                header.col(|ui| {
                                    ui.heading("Name");
                                });
                                header.col(|ui| {
                                    ui.heading("Details");
                                });
                                header.col(|ui| {
                                    ui.heading("Salary");
                                });
                                header.col(|ui| {
                                    ui.heading("Updated");
                                });
                                header.col(|ui| {
                                    ui.heading("Created");
                                });
                            })
                            .body(|mut body| {
                                for position in data {
                                    body.row(24.0, |mut row| {
                                        row.col(|ui| {
                                            ui.label(format!("{}", position.uuid));
                                        });
                                        row.col(|ui| {
                                            ui.label(position.name.clone());
                                        });
                                        row.col(|ui| {
                                            ui.label(
                                                if let Some(name) = position.details.clone() {
                                                    name
                                                } else {
                                                    String::new()
                                                },
                                            );
                                        });
                                        row.col(|ui| {
                                            ui.label(format!("{}₽.", position.salary));
                                        });
                                        row.col(|ui| {
                                            ui.label(format!("{}", position.meta.updated));
                                        });
                                        row.col(|ui| {
                                            ui.label(format!("{}", position.meta.created));
                                        });
                                    })
                                }
                            }),
                        Tab::Manufacturers { data } => TableBuilder::new(ui)
                            .striped(true)
                            .resizable(true)
                            .column(Size::exact(240.0))
                            .column(Size::initial(120.0))
                            .column(Size::exact(80.0))
                            .header(20.0, |mut header| {
                                header.col(|ui| {
                                    ui.heading("UUID");
                                });
                                header.col(|ui| {
                                    ui.heading("Name");
                                });
                                header.col(|ui| {
                                    ui.heading("Country");
                                });
                            })
                            .body(|mut body| {
                                for manufacturer in data {
                                    body.row(24.0, |mut row| {
                                        row.col(|ui| {
                                            ui.label(format!("{}", manufacturer.uuid));
                                        });
                                        row.col(|ui| {
                                            ui.label(manufacturer.name.clone());
                                        });
                                        row.col(|ui| {
                                            ui.label(manufacturer.country.clone());
                                        });
                                    })
                                }
                            }),
                        Tab::Services { data } => TableBuilder::new(ui)
                            .striped(true)
                            .resizable(true)
                            .column(Size::exact(240.0))
                            .column(Size::initial(120.0))
                            .column(Size::exact(120.0))
                            .column(Size::exact(120.0))
                            .column(Size::exact(210.0))
                            .column(Size::exact(210.0))
                            .header(20.0, |mut header| {
                                header.col(|ui| {
                                    ui.heading("UUID");
                                });
                                header.col(|ui| {
                                    ui.heading("Name");
                                });
                                header.col(|ui| {
                                    ui.heading("Description");
                                });
                                header.col(|ui| {
                                    ui.heading("Updated");
                                });
                                header.col(|ui| {
                                    ui.heading("Created");
                                });
                            })
                            .body(|mut body| {
                                for service in data {
                                    body.row(24.0, |mut row| {
                                        row.col(|ui| {
                                            ui.label(format!("{}", service.uuid));
                                        });
                                        row.col(|ui| {
                                            ui.label(service.name.clone());
                                        });
                                        row.col(|ui| {
                                            ui.label(
                                                if let Some(description) =
                                                    service.description.clone()
                                                {
                                                    description
                                                } else {
                                                    String::new()
                                                },
                                            );
                                        });
                                        row.col(|ui| {
                                            ui.label(format!("{}", service.meta.created));
                                        });
                                        row.col(|ui| {
                                            ui.label(format!("{}", service.meta.created));
                                        });
                                    })
                                }
                            }),
                        Tab::Suppliers { data } => TableBuilder::new(ui)
                            .striped(true)
                            .resizable(true)
                            .column(Size::exact(240.0))
                            .column(Size::initial(120.0))
                            .column(Size::initial(120.0))
                            .column(Size::initial(120.0))
                            .column(Size::initial(210.0))
                            .column(Size::exact(30.0))
                            .header(20.0, |mut header| {
                                header.col(|ui| {
                                    ui.heading("UUID");
                                });
                                header.col(|ui| {
                                    ui.heading("Name");
                                });
                                header.col(|ui| {
                                    ui.heading("IBAN");
                                });
                                header.col(|ui| {
                                    ui.heading("Swift");
                                });
                                header.col(|ui| {
                                    ui.heading("Address");
                                });
                                header.col(|ui| {
                                    ui.heading("Country");
                                }); // TODO: Add MetaTime
                            })
                            .body(|mut body| {
                                for supplier in data {
                                    body.row(24.0, |mut row| {
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
                                    })
                                }
                            }),
                    };
                });
            }
            TabHandler::Loading(_, tab) => {
                CentralPanel::default().show(ctx, |ui| {
                    ui.spinner();
                    ui.add_space(8.0);
                    ui.heading(format!("Loading \"{}\" table", tab.as_str()));
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

pub enum TabHandler {
    None,
    Loading(Request<(), Tab>, Tabs),
    Loaded(Tab),
}
