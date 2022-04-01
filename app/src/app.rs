use eframe::{
    egui::{
        global_dark_light_mode_switch, CentralPanel, Context, RichText, TextEdit, TextStyle,
        TopBottomPanel, Visuals, Window,
    },
    emath::Align2,
    epaint::Vec2,
    epi::{App as EApp, Frame, Storage},
};

pub struct App {
    state: AppScreen,
}

impl App {
    /// Creating app instance
    ///
    /// ## Authorization process
    /// 1. Tries to check and read session file (`profile.session`)
    /// 2. If first step fails then setup window will appear.
    ///    If remember were checked then user will skip auth step.
    ///    Else user will need to go through auth step.
    pub fn new() -> Self {
        // TODO: Profile session
        Self {
            state: AppScreen::setup(),
        }
    }
}

impl EApp for App {
    fn update(&mut self, ctx: &Context, _frame: &Frame) {
        let mut state = None;

        TopBottomPanel::top("top_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                global_dark_light_mode_switch(ui);

                if let AppScreen::Main = &mut self.state {
                    ui.separator();
                }
            })
        });

        match &mut self.state {
            AppScreen::Auth {
                login_input,
                password_input,
                remember_me,
            } => {
                Window::new("Authorization")
                    .resizable(false)
                    .collapsible(false)
                    .anchor(Align2::CENTER_CENTER, Vec2::ZERO) // FIX
                    .show(ctx, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.add(
                                TextEdit::singleline(login_input)
                                    .font(TextStyle::Heading)
                                    .hint_text("Login"),
                            );
                            ui.add_space(8.0);
                            ui.add(
                                TextEdit::singleline(password_input)
                                    .font(TextStyle::Heading)
                                    .hint_text("Password")
                                    .password(true),
                            );
                            ui.add_space(8.0);
                            ui.checkbox(remember_me, "Remember me");
                            ui.add_space(16.0);
                            if ui.button("Sign In").clicked() {
                                state = Some(AppScreen::Main);
                            }
                        });
                    });
            }
            AppScreen::Setup {
                address_input,
                user_input,
                password_input,
                db_name_input,
                admin_login_input,
                admin_password_input,
            } => {
                Window::new("Set Up")
                    .resizable(false)
                    .collapsible(false)
                    .anchor(Align2::CENTER_CENTER, Vec2::ZERO) // FIX
                    .show(ctx, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.label(RichText::new("Connection").text_style(TextStyle::Heading));
                            ui.separator();
                            ui.add_space(8.0);
                            ui.add(
                                TextEdit::singleline(address_input)
                                    .font(TextStyle::Heading)
                                    .hint_text("DB Address"),
                            );
                            ui.add_space(8.0);
                            ui.add(
                                TextEdit::singleline(user_input)
                                    .font(TextStyle::Heading)
                                    .hint_text("DB User"),
                            );
                            ui.add_space(8.0);
                            ui.add(
                                TextEdit::singleline(password_input)
                                    .font(TextStyle::Heading)
                                    .hint_text("DB Password")
                                    .password(true),
                            );
                            ui.add_space(8.0);
                            ui.add(
                                TextEdit::singleline(db_name_input)
                                    .font(TextStyle::Heading)
                                    .hint_text("DB Name"),
                            );
                            ui.add_space(16.0);
                            ui.label(RichText::new("Admin User").text_style(TextStyle::Heading));
                            ui.separator();
                            ui.add_space(8.0);
                            ui.add(
                                TextEdit::singleline(admin_login_input)
                                    .font(TextStyle::Heading)
                                    .hint_text("Admin login"),
                            );
                            ui.add_space(8.0);
                            ui.add(
                                TextEdit::singleline(admin_password_input)
                                    .font(TextStyle::Heading)
                                    .hint_text("Admin password")
                                    .password(true),
                            );
                            ui.add_space(16.0);
                            if ui.button("Proceed").clicked() {
                                state = Some(AppScreen::auth());
                            }
                        })
                    });
            }
            AppScreen::Main => {
                CentralPanel::default().show(ctx, |ui| {
                    ui.label("Main state [WIP]");
                });
            }
        }

        if let Some(new) = state {
            self.state = new;
        }
    }

    fn name(&self) -> &str {
        "CW App"
    }

    fn setup(&mut self, ctx: &Context, _frame: &Frame, _storage: Option<&dyn Storage>) {
        ctx.set_visuals(Visuals::dark());
    }
}

pub enum AppScreen {
    Auth {
        login_input: String,
        password_input: String,
        remember_me: bool,
    },
    Setup {
        address_input: String,
        user_input: String,
        password_input: String,
        db_name_input: String,
        admin_login_input: String,
        admin_password_input: String,
    },
    Main,
}

impl AppScreen {
    pub fn setup() -> Self {
        Self::Setup {
            address_input: String::new(),
            user_input: String::new(),
            password_input: String::new(),
            db_name_input: String::new(),
            admin_login_input: String::new(),
            admin_password_input: String::new(),
        }
    }

    pub fn auth() -> Self {
        Self::Auth {
            login_input: String::new(),
            password_input: String::new(),
            remember_me: false,
        }
    }
}
