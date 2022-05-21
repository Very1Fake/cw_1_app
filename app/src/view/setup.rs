use std::sync::Arc;

use cw_core::sqlx::PgPool;
use eframe::{
    egui::{ComboBox, Context, DragValue, RichText, TextEdit, TextStyle, Window},
    emath::{Align2, Vec2},
    epaint::Color32,
};
use tokio::{runtime::Runtime, sync::mpsc::channel};

use crate::{
    model::{
        config::{Config, Connection, SslMode},
        request::{Request, RequestStatus},
    },
    utils::{open_pool, Pool},
};

use super::ViewResponse;

pub struct SetupView {
    // UI
    host_input: String,
    user_input: String,
    password_input: String,
    database_input: String,
    ssl_mode: SslMode,
    min_pool: u32,
    max_pool: u32,

    // Internals
    is_reactive: bool,
    processing: Option<Request<String, Pool>>,
    error: Option<String>,
}

impl SetupView {
    fn is_reactive(&self) -> bool {
        !self.host_input.is_empty()
            && !self.user_input.is_empty()
            && !self.database_input.is_empty()
    }

    pub fn from_config(config: &Config) -> Self {
        let mut this = Self {
            database_input: String::from("cw1_db"),
            ..Default::default()
        };

        if let Some(connection) = &config.connection {
            this.host_input = connection.host.clone();
            this.user_input = connection.user.clone();
            this.password_input = connection.password.clone();
            this.database_input = connection.database.clone();
            this.min_pool = connection.min_pool;
            this.max_pool = connection.max_pool;
        }

        this
    }

    pub fn reactive(config: &Config, runtime: &Runtime) -> Self {
        let mut this = Self::from_config(config);

        // Check for empty inputs
        if this.is_reactive() {
            this.is_reactive = true;
            this.start_processing(runtime);
        }

        this
    }

    fn start_processing(&mut self, runtime: &Runtime) {
        let uri = format!(
            "postgres://{}:{}@{}/{}",
            self.user_input, self.password_input, self.host_input, self.database_input
        );
        let ssl_mode = self.ssl_mode;
        let bound = (self.min_pool, self.max_pool);
        let (tx, rx) = channel(2);
        self.processing = Some(Request::new(
            runtime.spawn(async move {
                tx.send(String::from("Connecting")).await?;
                let pool = open_pool(uri, ssl_mode, bound).await?;
                tx.send(String::from("Connected")).await?;
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
    ) -> ViewResponse<Arc<PgPool>> {
        let mut response = ViewResponse::Remain;
        Window::new(if self.processing.is_some() {
            "Setup/Processing"
        } else {
            "Setup"
        })
        .resizable(false)
        .collapsible(false)
        .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
        .show(ctx, |ui| {
            if let Some(mut request) = self.processing.take() {
                ui.vertical_centered(|ui| {
                    ui.spinner();
                    ui.add_space(8.0);
                    match request.peek(runtime).status.take() {
                        RequestStatus::Last(item) => {
                            ui.label(if let Some(status) = item {
                                RichText::new(status).heading()
                            } else {
                                RichText::new("Waiting").heading()
                            });
                            self.processing = Some(request);
                        }
                        RequestStatus::Finished(result) => match result {
                            Ok(pool) => {
                                config.connection = Some(Connection {
                                    host: self.host_input.clone(),
                                    user: self.user_input.clone(),
                                    password: self.password_input.clone(),
                                    database: self.database_input.clone(),
                                    ssl_mode: self.ssl_mode,
                                    min_pool: self.min_pool,
                                    max_pool: self.max_pool,
                                });
                                response = ViewResponse::next(pool, self.is_reactive);
                            }
                            Err(err) => {
                                self.error = Some(format!("{err}"));
                            }
                        },
                    };
                });
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
                        TextEdit::singleline(&mut self.database_input)
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
                        ui.heading("Minimum connections in pool: ");
                        ui.add(
                            DragValue::new(&mut self.min_pool)
                                .clamp_range(0..=16_u32)
                                .speed(0.1)
                                .suffix(" conns."),
                        );
                        ui.heading("Maximum connections in pool: ");
                        ui.add(
                            DragValue::new(&mut self.max_pool)
                                .clamp_range(0..=16_u32)
                                .speed(0.1)
                                .suffix(" conns."),
                        );
                    });
                    ui.add_space(16.0);
                    if ui.button("Proceed").clicked() {
                        self.start_processing(runtime)
                    }
                });
            }
        });

        response
    }
}

impl Default for SetupView {
    fn default() -> Self {
        Self {
            host_input: Default::default(),
            user_input: Default::default(),
            password_input: Default::default(),
            database_input: Default::default(),
            ssl_mode: Default::default(),
            min_pool: 1,
            max_pool: 16,
            is_reactive: Default::default(),
            processing: Default::default(),
            error: Default::default(),
        }
    }
}
