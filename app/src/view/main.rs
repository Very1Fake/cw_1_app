use std::{mem::replace, sync::Arc};

use cw_core::{
    tables::{Component, Manufacturer, Person, Phone, Position, Service, Supplier},
    uuid::Uuid,
};
use eframe::{
    egui::{Button, Context, RichText, TopBottomPanel, Window},
    emath::{Align2, Vec2},
    epaint::Color32,
};
use egui_extras::Size;
use tokio::runtime::Runtime;

use crate::{
    model::{
        request::{Request, RequestStatus},
        user::User,
    },
    utils::Pool,
};

use super::table::{
    Table, TableAccess, TableData, TableWindow, WindowState, WindowStorage, BUTTON_WIDTH,
    COUNTRY_WIDTH, ID_WIDTH, TIMESTAMP_WIDTH, UUID_WIDTH,
};

pub struct MainView {
    user: User,
    windows: WindowStorage,
    delete_prompt: DeletePrompt,
}

impl MainView {
    pub fn new(user: User) -> Self {
        let windows = TableWindow::all_by_role(user.account.role);

        Self {
            user,
            windows,
            delete_prompt: DeletePrompt::None,
        }
    }

    pub fn update(&mut self, ctx: &Context, runtime: &Runtime, pool: Pool) {
        TopBottomPanel::top("main_tabs").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                for (window, (open, _, state)) in &mut self.windows {
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

        if !self.delete_prompt.is_none() {
            Window::new("Delete confirmation")
                .collapsible(false)
                .resizable(false)
                .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
                .show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        // FIX: WTF
                        match self.delete_prompt.take() {
                            DeletePrompt::Error(msg) => {
                                ui.collapsing(
                                    RichText::new("An error occurred")
                                        .heading()
                                        .color(Color32::RED),
                                    |ui| ui.label(&msg),
                                );
                                ui.add_space(8.0);
                                self.delete_prompt = if ui.button("Close").clicked() {
                                    DeletePrompt::None
                                } else {
                                    DeletePrompt::Error(msg)
                                }
                            }
                            DeletePrompt::Loading((window, mut request)) => {
                                self.delete_prompt = match request.peek(runtime).status.take() {
                                    RequestStatus::Finished(result) => match result {
                                        Ok(_) => {
                                            self.windows.get_mut(&window).unwrap().2 =
                                                WindowState::load(
                                                    runtime,
                                                    Arc::clone(&pool),
                                                    window,
                                                );
                                            DeletePrompt::None
                                        }
                                        Err(err) => DeletePrompt::Error(format!("{err}")),
                                    },
                                    _ => {
                                        ui.spinner();
                                        ui.add_space(8.0);
                                        ui.label("Deleting row");
                                        DeletePrompt::Loading((window, request))
                                    }
                                }
                            }
                            DeletePrompt::Confirm((uuid, window)) => {
                                ui.label(format!(
                                    "Are you sure you want to delete '{}' from '{}' table",
                                    uuid,
                                    window.as_str()
                                ));
                                if ui.add(Button::new("Delete").fill(Color32::RED)).clicked() {
                                    let d_pool = Arc::clone(&pool);
                                    let d_uuid = uuid;
                                    self.delete_prompt = DeletePrompt::Loading((
                                        window,
                                        Request::simple(runtime, move || async move {
                                            match window {
                                                TableWindow::People => {
                                                    Person::delete_by_uuid(d_uuid)
                                                        .execute(&*d_pool)
                                                        .await?;
                                                }
                                                TableWindow::Positions => {
                                                    Position::delete_by_uuid(d_uuid)
                                                        .execute(&*d_pool)
                                                        .await?;
                                                }
                                                TableWindow::Manufacturers => {
                                                    Manufacturer::delete_by_uuid(d_uuid)
                                                        .execute(&*d_pool)
                                                        .await?;
                                                }
                                                TableWindow::Services => {
                                                    Service::delete_by_uuid(d_uuid)
                                                        .execute(&*d_pool)
                                                        .await?;
                                                }
                                                TableWindow::Suppliers => {
                                                    Supplier::delete_by_uuid(d_uuid)
                                                        .execute(&*d_pool)
                                                        .await?;
                                                }
                                                TableWindow::Phones => {
                                                    Phone::delete_by_uuid(d_uuid)
                                                        .execute(&*d_pool)
                                                        .await?;
                                                }
                                                TableWindow::Components => {
                                                    Component::delete_by_uuid(d_uuid)
                                                        .execute(&*d_pool)
                                                        .await?;
                                                }
                                            }
                                            Ok(())
                                        }),
                                    ))
                                } else {
                                    ui.add_space(8.0);
                                    self.delete_prompt = if ui.button("Cancel").clicked() {
                                        DeletePrompt::None
                                    } else {
                                        DeletePrompt::Confirm((uuid, window))
                                    }
                                }
                            }
                            DeletePrompt::None => unreachable!(),
                        };
                    })
                });
        }

        self.windows
            .iter_mut()
            .map(|(window, (open, access, state))| {
                if *open != state.is_visible() && !*open {
                    *state = WindowState::None;
                }
                (window, (open, access, state))
            })
            .for_each(|(window, (open, access, state))| {
                Window::new(window.as_str())
                    .open(open)
                    .resizable(true)
                    .enabled(self.delete_prompt.is_none())
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
                                        Size::exact(TIMESTAMP_WIDTH),
                                        Size::exact(TIMESTAMP_WIDTH),
                                        Size::exact(BUTTON_WIDTH),
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
                                            row.col(|ui| {
                                                if *access >= TableAccess::Delete {
                                                    if ui.button("🗑").clicked() {
                                                        self.delete_prompt = DeletePrompt::Confirm(
                                                            (person.uuid, *window),
                                                        );
                                                    }
                                                }
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
                                        Size::exact(ID_WIDTH),
                                        Size::exact(UUID_WIDTH),
                                        Size::initial(120.0),
                                        Size::initial(120.0),
                                        Size::initial(80.0),
                                        Size::exact(TIMESTAMP_WIDTH),
                                        Size::exact(TIMESTAMP_WIDTH),
                                        Size::exact(BUTTON_WIDTH),
                                    ],
                                    &[
                                        "ID", "UUID", "Name", "Details", "Salary", "Updated",
                                        "Created",
                                    ],
                                    (data.len(), |index, mut row| match data.get(index) {
                                        Some(position) => {
                                            row.col(|ui| {
                                                ui.label(index.to_string());
                                            });
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
                                            row.col(|ui| {
                                                if *access >= TableAccess::Delete {
                                                    if ui.button("🗑").clicked() {
                                                        self.delete_prompt = DeletePrompt::Confirm(
                                                            (position.uuid, *window),
                                                        );
                                                    }
                                                }
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
                                        Size::exact(ID_WIDTH),
                                        Size::exact(UUID_WIDTH),
                                        Size::initial(120.0),
                                        Size::exact(50.0),
                                        Size::exact(BUTTON_WIDTH),
                                    ],
                                    &["ID", "UUID", "Name", "Country"],
                                    (data.len(), |index, mut row| match data.get(index) {
                                        Some(manufacturer) => {
                                            row.col(|ui| {
                                                ui.label(index.to_string());
                                            });
                                            row.col(|ui| {
                                                ui.label(format!("{}", manufacturer.uuid));
                                            });
                                            row.col(|ui| {
                                                ui.label(manufacturer.name.clone());
                                            });
                                            row.col(|ui| {
                                                ui.label(manufacturer.country.clone());
                                            });
                                            row.col(|ui| {
                                                if *access >= TableAccess::Delete {
                                                    if ui.button("🗑").clicked() {
                                                        self.delete_prompt = DeletePrompt::Confirm(
                                                            (manufacturer.uuid, *window),
                                                        );
                                                    }
                                                }
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
                                        Size::exact(ID_WIDTH),
                                        Size::exact(UUID_WIDTH),
                                        Size::initial(120.0),
                                        Size::initial(120.0),
                                        Size::exact(TIMESTAMP_WIDTH),
                                        Size::exact(TIMESTAMP_WIDTH),
                                        Size::exact(BUTTON_WIDTH),
                                    ],
                                    &["ID", "UUID", "Name", "Description", "Updated", "Created"],
                                    (data.len(), |index, mut row| match data.get(index) {
                                        Some(service) => {
                                            row.col(|ui| {
                                                ui.label(index.to_string());
                                            });
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
                                            row.col(|ui| {
                                                if *access >= TableAccess::Delete {
                                                    if ui.button("🗑").clicked() {
                                                        self.delete_prompt = DeletePrompt::Confirm(
                                                            (service.uuid, *window),
                                                        );
                                                    }
                                                }
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
                                        Size::exact(ID_WIDTH),
                                        Size::exact(UUID_WIDTH),
                                        Size::initial(120.0),
                                        Size::exact(235.0),
                                        Size::initial(120.0),
                                        Size::initial(210.0),
                                        Size::exact(COUNTRY_WIDTH),
                                        Size::exact(BUTTON_WIDTH),
                                    ],
                                    &["ID", "UUID", "Name", "IBAN", "Swift", "Address", "Country"],
                                    (data.len(), |index, mut row| match data.get(index) {
                                        Some(supplier) => {
                                            row.col(|ui| {
                                                ui.label(index.to_string());
                                            });
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
                                            row.col(|ui| {
                                                if *access >= TableAccess::Delete {
                                                    if ui.button("🗑").clicked() {
                                                        self.delete_prompt = DeletePrompt::Confirm(
                                                            (supplier.uuid, *window),
                                                        );
                                                    }
                                                }
                                            });
                                        }
                                        None => {
                                            row.col(|ui| {
                                                ui.label("Error while indexing");
                                            });
                                        }
                                    }),
                                ),
                                TableData::Phones { data } => Table::draw(
                                    ui,
                                    &[
                                        Size::exact(ID_WIDTH),
                                        Size::exact(UUID_WIDTH),
                                        Size::initial(120.0),
                                        Size::initial(120.0),
                                        Size::initial(120.0),
                                        Size::initial(120.0),
                                        Size::initial(120.0),
                                        Size::initial(120.0),
                                        Size::exact(TIMESTAMP_WIDTH),
                                        Size::exact(TIMESTAMP_WIDTH),
                                        Size::exact(BUTTON_WIDTH),
                                    ],
                                    &[
                                        "ID",
                                        "UUID",
                                        "Owner",
                                        "IMEI",
                                        "WiFi",
                                        "Bluetooth",
                                        "Model",
                                        "Color",
                                        "Updated",
                                        "Created",
                                    ],
                                    (data.len(), |index, mut row| match data.get(index) {
                                        Some(phone) => {
                                            row.col(|ui| {
                                                ui.label(index.to_string());
                                            });
                                            row.col(|ui| {
                                                ui.label(format!("{}", phone.uuid));
                                            });
                                            row.col(|ui| {
                                                ui.label(phone.owner.clone());
                                            });
                                            row.col(|ui| {
                                                ui.label(phone.imei.clone());
                                            });
                                            row.col(|ui| {
                                                ui.label(format!("{}", phone.wifi.clone()));
                                            });
                                            row.col(|ui| {
                                                ui.label(format!("{}", phone.bluetooth.clone()));
                                            });
                                            row.col(|ui| {
                                                ui.label(phone.model.clone());
                                            });
                                            row.col(|ui| {
                                                ui.label(phone.color.as_str());
                                            });
                                            row.col(|ui| {
                                                ui.label(format!("{}", phone.meta.updated));
                                            });
                                            row.col(|ui| {
                                                ui.label(format!("{}", phone.meta.created));
                                            });
                                            row.col(|ui| {
                                                if *access >= TableAccess::Delete {
                                                    if ui.button("🗑").clicked() {
                                                        self.delete_prompt = DeletePrompt::Confirm(
                                                            (phone.uuid, *window),
                                                        );
                                                    }
                                                }
                                            });
                                        }
                                        None => {
                                            row.col(|ui| {
                                                ui.label("Error while indexing");
                                            });
                                        }
                                    }),
                                ),
                                TableData::Components { data } => Table::draw(
                                    ui,
                                    &[
                                        Size::exact(ID_WIDTH),
                                        Size::exact(UUID_WIDTH),
                                        Size::initial(120.0),
                                        Size::initial(120.0),
                                        Size::initial(120.0),
                                        Size::initial(120.0),
                                        Size::exact(BUTTON_WIDTH),
                                    ],
                                    &[
                                        "ID",
                                        "UUID",
                                        "Name",
                                        "Kind",
                                        "Phone Model",
                                        "Manufacturer",
                                    ],
                                    (data.len(), |index, mut row| match data.get(index) {
                                        Some(component) => {
                                            row.col(|ui| {
                                                ui.label(index.to_string());
                                            });
                                            row.col(|ui| {
                                                ui.label(format!("{}", component.uuid));
                                            });
                                            row.col(|ui| {
                                                ui.label(component.name.clone());
                                            });
                                            row.col(|ui| {
                                                ui.label(component.kind.clone());
                                            });
                                            row.col(|ui| {
                                                ui.label(component.model.clone());
                                            });
                                            row.col(|ui| {
                                                ui.label(component.manufacturer.clone());
                                            });
                                            row.col(|ui| {
                                                if *access >= TableAccess::Delete {
                                                    if ui.button("🗑").clicked() {
                                                        self.delete_prompt = DeletePrompt::Confirm(
                                                            (component.uuid, *window),
                                                        );
                                                    }
                                                }
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
                                ui.collapsing("An error occurred while loading table", |ui| {
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

pub enum DeletePrompt {
    None,
    Confirm((Uuid, TableWindow)),
    Loading((TableWindow, Request<(), ()>)),
    Error(String),
}

impl DeletePrompt {
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    pub fn take(&mut self) -> Self {
        replace(self, Self::None)
    }
}
