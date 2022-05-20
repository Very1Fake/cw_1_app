use std::{sync::Arc, time::Duration};

use cw_core::sqlx::PgPool;
use eframe::{
    egui::{ComboBox, Context, RichText, TextEdit, TextStyle, Window},
    emath::{Align2, Vec2},
    epaint::Color32,
};
use tokio::{runtime::Runtime, sync::mpsc::channel, time::sleep};

use crate::{
    model::{
        config::{Config, Connection, SslMode},
        request::{Request, RequestStatus},
    },
    utils::{open_pool, Pool},
};

#[derive(Default)]
pub struct SetupView {
    // UI
    host_input: String,
    user_input: String,
    password_input: String,
    db_name_input: String,
    ssl_mode: SslMode,

    // Internals
    processing: Option<Request<SetupSteps, Pool>>,
    error: Option<String>,
}

impl SetupView {
    pub fn new_with_config(config: &Config, runtime: &Runtime) -> Self {
        let mut this = Self {
            db_name_input: String::from("cw1_db"),
            ..Default::default()
        };

        if let Some(connection) = &config.connection {
            this.host_input = connection.host.clone();
            this.user_input = connection.user.clone();
            this.password_input = connection.password.clone();
            this.db_name_input = connection.database.clone();
            this.start_processing(runtime)
        }

        this
    }

    fn start_processing(&mut self, runtime: &Runtime) {
        let uri = format!(
            "postgres://{}:{}@{}/{}",
            self.user_input, self.password_input, self.host_input, self.db_name_input
        );
        let ssl_mode = self.ssl_mode;
        let (tx, rx) = channel(2);
        self.processing = Some(Request::new(
            runtime.spawn(async move {
                tx.send(SetupSteps::Text(String::from("Connecting")))
                    .await?;
                let pool = open_pool(uri, ssl_mode).await?;
                tx.send(SetupSteps::Text(String::from("Connected"))).await?;
                sleep(Duration::from_secs(1)).await;

                Ok(Arc::new(pool))
            }),
            rx,
        ))
    }

    pub fn update(
        &mut self,
        ctx: &Context,
        config: &mut Config,
        runtime: &Runtime,
    ) -> Option<Arc<PgPool>> {
        let mut back = false;
        let mut forward = None;
        Window::new(if self.processing.is_some() {
            "Setup/Processing"
        } else {
            "Setup"
        })
        .resizable(false)
        .collapsible(false)
        .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
        .show(ctx, |ui| {
            if self.processing.is_some() {
                ui.vertical_centered(|ui| {
                    ui.spinner();
                    match &self.processing.as_mut().unwrap().peek(runtime).status {
                        RequestStatus::Last(status) => {
                            ui.label(if let Some(SetupSteps::Text(text)) = status {
                                RichText::new(text).heading()
                            } else {
                                RichText::new("Waiting").heading()
                            });
                        }
                        RequestStatus::Finished(result) => match result {
                            Ok(pool) => {
                                config.connection = Some(Connection {
                                    host: self.host_input.clone(),
                                    user: self.user_input.clone(),
                                    password: self.password_input.clone(),
                                    database: self.db_name_input.clone(),
                                    ssl_mode: self.ssl_mode,
                                });
                                forward = Some(Arc::clone(pool))
                            }
                            Err(err) => {
                                self.error = Some(format!("{err}"));
                                back = true;
                            }
                        },
                    };
                })
            } else {
                ui.vertical_centered(|ui| {
                    if let Some(error) = &self.error {
                        ui.collapsing(
                            RichText::new("Error Occurred")
                                .heading()
                                .color(Color32::RED),
                            |col| col.label(error),
                        );
                        ui.add_space(16.0);
                    }

                    ui.label(RichText::new("Connection").text_style(TextStyle::Heading));
                    ui.separator();
                    ui.add_space(8.0);
                    ui.add(
                        TextEdit::singleline(&mut self.host_input)
                            .font(TextStyle::Heading)
                            .hint_text("DB Host"),
                    );
                    ui.add_space(8.0);
                    ui.add(
                        TextEdit::singleline(&mut self.user_input)
                            .font(TextStyle::Heading)
                            .hint_text("DB User"),
                    );
                    ui.add_space(8.0);
                    ui.add(
                        TextEdit::singleline(&mut self.password_input)
                            .font(TextStyle::Heading)
                            .hint_text("DB Password")
                            .password(true),
                    );
                    ui.add_space(8.0);
                    ui.add(
                        TextEdit::singleline(&mut self.db_name_input)
                            .font(TextStyle::Heading)
                            .hint_text("DB Name"),
                    );
                    ui.add_space(16.0);
                    ui.collapsing(RichText::new("More options").heading(), |ui| {
                        ui.heading("SSL Mode: ");
                        ComboBox::from_id_source("ssl_mode")
                            .selected_text(self.ssl_mode.as_str())
                            .show_ui(ui, |combo| {
                                SslMode::ALL.iter().for_each(|mode| {
                                    combo.selectable_value(
                                        &mut self.ssl_mode,
                                        *mode,
                                        mode.as_str(),
                                    );
                                })
                            });
                    });
                    ui.add_space(16.0);
                    if ui.button("Proceed").clicked() {
                        self.start_processing(runtime)
                    }
                })
            }
        });

        if back {
            self.processing = None;
        }
        forward
    }
}

#[derive(Debug)]
enum SetupSteps {
    Text(String),
}
