use anyhow::bail;
use cw_core::{
    argon2::{Argon2, PasswordHash, PasswordVerifier},
    sqlx::Error,
    tables::{Account, LaborContract, Person, Staff},
};
use eframe::{
    egui::{Button, Checkbox, Context, RichText, TextEdit, TextStyle, Window},
    emath::{Align2, Vec2},
    epaint::Color32,
};
use tokio::runtime::Runtime;

use crate::{
    model::{
        config::{Account as ConfigAccount, Config},
        request::{Request, RequestStatus},
        user::User,
    },
    utils::Pool,
};

#[derive(Default)]
pub struct AuthView {
    // UI
    login_input: String,
    password_input: String,
    remember_me: bool,

    // Internals
    processing: Option<Request<(), User>>,
    error: Option<String>,
}

impl AuthView {
    pub fn new_with_config(config: &Config, runtime: &Runtime, pool: Pool) -> Self {
        let mut this = Self::default();

        if let Some(account) = &config.account {
            this.login_input = account.login.clone();
            this.password_input = account.password.clone();
            this.remember_me = true;
            this.start_processing(runtime, pool);
        }

        this
    }

    fn start_processing(&mut self, runtime: &Runtime, pool: Pool) {
        let login = self.login_input.clone();
        let password = self.password_input.clone();
        self.processing = Some(Request::simple(runtime.spawn(async move {
            let account = match Account::get_by_login(login).fetch_one(&*pool).await {
                Ok(account) => account,
                Err(Error::RowNotFound) => return bail!("Account not found"),
                Err(err) => return bail!(err),
            };

            let hash = if let Ok(hash) = PasswordHash::new(&account.password) {
                hash
            } else {
                return bail!("Can't parse password hash from db");
            };

            match Argon2::default().verify_password(password.as_bytes(), &hash) {
                Ok(_) => {
                    let staff = Staff::get_by_uuid(account.staff).fetch_one(&*pool).await?;
                    let labor_contract = LaborContract::get_by_uuid(staff.contract)
                        .fetch_one(&*pool)
                        .await?;
                    let person = Person::get_by_uuid(labor_contract.person)
                        .fetch_one(&*pool)
                        .await?;
                    Ok(User {
                        account,
                        staff,
                        labor_contract,
                        person,
                    })
                }
                Err(err) => {
                    bail!(err)
                }
            }
        })))
    }

    pub fn update(
        &mut self,
        ctx: &Context,
        config: &mut Config,
        runtime: &Runtime,
        pool: Pool,
    ) -> Option<User> {
        let mut forward = None;
        let enabled = self.processing.is_none();

        Window::new("Authorization")
            .resizable(false)
            .collapsible(false)
            .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
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

                    ui.add_space(8.0);
                    ui.add(
                        TextEdit::singleline(&mut self.login_input)
                            .font(TextStyle::Heading)
                            .hint_text("Login")
                            .interactive(enabled),
                    );
                    ui.add_space(8.0);
                    ui.add(
                        TextEdit::singleline(&mut self.password_input)
                            .font(TextStyle::Heading)
                            .hint_text("Password")
                            .password(true)
                            .interactive(enabled),
                    );
                    ui.add_space(8.0);
                    ui.add_enabled(enabled, Checkbox::new(&mut self.remember_me, "Remember me"));
                    ui.add_space(16.0);
                    if ui.add_enabled(enabled, Button::new("Sign In")).clicked() {
                        self.start_processing(runtime, pool)
                    }
                });
            });
        {
            let mut failed = false;
            if let Some(request) = &mut self.processing {
                if let RequestStatus::Finished(result) = &request.peek(runtime).status {
                    match result {
                        Ok(user) => {
                            if self.remember_me {
                                config.account = Some(ConfigAccount {
                                    login: self.login_input.clone(),
                                    password: self.password_input.clone(),
                                })
                            };
                            forward = Some(user.clone())
                        }
                        Err(err) => {
                            self.error = Some(format!("{err}"));
                            failed = true;
                        }
                    }
                }
            }

            if failed {
                self.processing = None;
            }
        }
        forward
    }
}
