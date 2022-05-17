use std::{sync::Arc, time::Duration};

use cw_core::sqlx::PgPool;
use eframe::{
    egui::{Context, RichText, TextEdit, TextStyle, Window},
    emath::{Align2, Vec2},
    epaint::Color32,
};
use tokio::{runtime::Runtime, sync::mpsc::channel, time::sleep};

use crate::{
    model::request::{Request, RequestStatus},
    utils::{open_pool, Pool, SslMode},
};

pub struct SetupView {
    host_input: String,
    user_input: String,
    password_input: String,
    db_name_input: String,
    // admin_login_input: String,
    // admin_password_input: String,
    progress: Option<Request<SetupSteps, Pool>>,
    error: Option<String>,
}

impl SetupView {
    pub fn new() -> Self {
        Self {
            host_input: String::new(),
            user_input: String::new(),
            password_input: String::new(),
            db_name_input: String::from("cw1_db"),
            progress: None,
            error: None,
        }
    }

    pub fn update(&mut self, ctx: &Context, runtime: &Runtime) -> Option<Arc<PgPool>> {
        if self.progress.is_some() {
            let mut back = false;
            let mut forward = None;
            Window::new("Setup/Processing")
                .resizable(false)
                .collapsible(false)
                .anchor(Align2::CENTER_CENTER, Vec2::ZERO) // FIX
                .show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.spinner();
                        match &self.progress.as_mut().unwrap().peek(runtime).status {
                            RequestStatus::Last(status) => {
                                ui.label(if let Some(SetupSteps::Text(text)) = status {
                                    RichText::new(text).heading()
                                } else {
                                    RichText::new("Waiting").heading()
                                });
                            }
                            RequestStatus::Finished(result) => match result {
                                Ok(pool) => forward = Some(Arc::clone(pool)),
                                Err(err) => {
                                    self.error = Some(format!("{err}"));
                                    back = true;
                                }
                            },
                        };
                    })
                });

            if back {
                self.progress = None;
            }
            forward
        } else {
            Window::new("Setup")
                .resizable(false)
                .collapsible(false)
                .anchor(Align2::CENTER_CENTER, Vec2::ZERO) // FIX
                .show(ctx, |ui| {
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
                        // ui.label(RichText::new("Admin User").text_style(TextStyle::Heading));
                        // ui.separator();
                        // ui.add_space(8.0);
                        // ui.add(
                        //     TextEdit::singleline(&mut self.admin_login_input)
                        //         .font(TextStyle::Heading)
                        //         .hint_text("Admin login"),
                        // );
                        // ui.add_space(8.0);
                        // ui.add(
                        //     TextEdit::singleline(&mut self.admin_password_input)
                        //         .font(TextStyle::Heading)
                        //         .hint_text("Admin password")
                        //         .password(true),
                        // );
                        // ui.add_space(16.0);
                        if ui.button("Proceed").clicked() {
                            self.progress = {
                                let uri = format!(
                                    "postgres://{}:{}@{}/{}",
                                    self.user_input,
                                    self.password_input,
                                    self.host_input,
                                    self.db_name_input
                                );
                                println!("Uri: {uri}");
                                let (tx, rx) = channel(2);
                                Some(Request::new(
                                    runtime.spawn(async move {
                                        tx.send(SetupSteps::Text(String::from("Connecting")))
                                            .await?;
                                        let pool = open_pool(uri, SslMode::Allow).await?;
                                        tx.send(SetupSteps::Text(String::from("Connected")))
                                            .await?;
                                        sleep(Duration::from_secs(1)).await;

                                        Ok(Arc::new(pool))
                                    }),
                                    rx,
                                ))
                            }
                        }
                    })
                });
            None
        }
    }
}

#[derive(Debug)]
enum SetupSteps {
    Text(String),
}
