use std::collections::BTreeMap;

use cw_core::{
    tables::{Manufacturer, Person, Position, Service, Supplier},
    types::AccountRole,
    views::{ComponentBeautified, PhoneBeautified},
};
use eframe::egui::{TextStyle, Ui};
use egui_extras::{Size, TableBuilder, TableRow};
use tokio::runtime::Runtime;

use crate::{model::request::Request, utils::Pool};

pub type WindowStorage = BTreeMap<TableWindow, (bool, TableAccess, WindowState)>;

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
    Phones { data: Vec<PhoneBeautified> },
    Components { data: Vec<ComponentBeautified> },
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub enum TableWindow {
    People,
    Positions,
    Manufacturers,
    Services,
    Suppliers,
    Phones,
    Components,
}

impl TableWindow {
    pub const ALL: &'static [Self] = &[
        Self::People,
        Self::Positions,
        Self::Manufacturers,
        Self::Services,
        Self::Suppliers,
        Self::Phones,
        Self::Components,
    ];

    pub fn all_by_role(role: AccountRole) -> WindowStorage {
        let map = BTreeMap::from_iter(Self::ALL.iter().filter_map(|window| {
            if let Some((access, _)) = window
                .allowed_roles()
                .iter()
                .find(|(_, t_role)| role == *t_role)
            {
                Some((*window, (false, *access, WindowState::None)))
            } else {
                None
            }
        }));

        map
    }

    pub fn allowed_roles(&self) -> &[(TableAccess, AccountRole)] {
        match self {
            Self::People => &[
                (TableAccess::Full, AccountRole::Admin),
                (TableAccess::Edit, AccountRole::Accountant),
                (TableAccess::Create, AccountRole::Shopman),
            ],
            Self::Positions => &[
                (TableAccess::Full, AccountRole::Admin),
                (TableAccess::Edit, AccountRole::Accountant),
                (TableAccess::Create, AccountRole::HR),
            ],
            Self::Manufacturers => &[
                (TableAccess::Full, AccountRole::Admin),
                (TableAccess::Edit, AccountRole::Manager),
                (TableAccess::View, AccountRole::WarehouseWorker),
            ],
            Self::Services => &[
                (TableAccess::Full, AccountRole::Admin),
                (TableAccess::Full, AccountRole::Manager),
                (TableAccess::View, AccountRole::Serviceman),
            ],
            Self::Suppliers => &[
                (TableAccess::Full, AccountRole::Admin),
                (TableAccess::Edit, AccountRole::Manager),
                (TableAccess::View, AccountRole::Accountant),
            ],
            Self::Phones => &[
                (TableAccess::Full, AccountRole::Admin),
                (TableAccess::Edit, AccountRole::Serviceman),
                (TableAccess::Create, AccountRole::Shopman),
            ],
            Self::Components => &[
                (TableAccess::Full, AccountRole::Admin),
                (TableAccess::Full, AccountRole::Manager),
                (TableAccess::View, AccountRole::Serviceman),
                (TableAccess::View, AccountRole::WarehouseWorker),
            ],
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::People => "People",
            Self::Positions => "Positions",
            Self::Manufacturers => "Manufacturers",
            Self::Services => "Services",
            Self::Suppliers => "Suppliers",
            Self::Phones => "Phones",
            Self::Components => "Components",
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
                    TableWindow::Phones => TableData::Phones {
                        data: PhoneBeautified::get_all().fetch_all(&*pool).await?,
                    },
                    TableWindow::Components => TableData::Components {
                        data: ComponentBeautified::get_all().fetch_all(&*pool).await?,
                    },
                })
            }),
            window,
        )
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub enum TableAccess {
    View = 0,
    Create,
    Delete,
    Edit,
    Full,
}
