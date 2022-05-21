use std::sync::Arc;

use anyhow::Context as AnyContext;
use eframe::{
    egui::{global_dark_light_mode_switch, Context, TopBottomPanel, Visuals},
    glow, App as EApp, CreationContext, Frame,
};
use tokio::runtime::Runtime;
use tracing::trace;

use crate::{
    model::config::Config,
    utils::Pool,
    view::{AppViews, ViewResponse},
};

pub struct App {
    view: AppViews,
    runtime: Runtime,
    pool: Option<Pool>,
    config: Config,
}

impl App {
    /// Creating app instance
    ///
    /// ## Authorization process
    /// 1. Tries to check and read session file (`profile.session`)
    /// 2. If first step fails then setup window will appear.
    ///    If remember were checked then user will skip auth step.
    ///    Else user will need to go through auth step.
    pub fn new(cc: &CreationContext<'_>, runtime: Runtime) -> Self {
        let config = Config::load().context("While loading config").unwrap();

        cc.egui_ctx.set_visuals(Visuals::dark());

        Self {
            view: AppViews::setup_reactive(&config, &runtime),
            runtime,
            pool: None,
            config,
        }
    }
}

impl EApp for App {
    fn on_exit(&mut self, _gl: &glow::Context) {
        trace!("Trying to save config");
        self.config.save().context("While saving config").unwrap();
    }

    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        TopBottomPanel::top("top_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                global_dark_light_mode_switch(ui);

                if !matches!(self.view, AppViews::Setup(_)) {
                    ui.separator();
                    ui.menu_button("Menu", |ui| {
                        if matches!(self.view, AppViews::Main(_)) && ui.button("Logout").clicked() {
                            self.view = AppViews::auth(&self.config);
                            ui.close_menu();
                        }
                        if ui.button("Disconnect").clicked() {
                            self.pool.take();
                            self.view = AppViews::setup(&self.config);
                            ui.close_menu();
                        }
                    });
                }
            })
        });

        match &mut self.view {
            AppViews::Auth(view) => {
                if let ViewResponse::Next((user, _)) = view.update(
                    ctx,
                    &mut self.config,
                    &self.runtime,
                    Arc::clone(self.pool.as_ref().expect("Unwrapping pool in auth view")),
                ) {
                    self.view = AppViews::main(user)
                }
            }
            AppViews::Setup(view) => {
                if let ViewResponse::Next((pool, _)) =
                    view.update(ctx, &mut self.config, &self.runtime)
                {
                    self.pool = Some(pool);
                    self.view = AppViews::auth_reactive(
                        &self.config,
                        &self.runtime,
                        Arc::clone(
                            self.pool
                                .as_ref()
                                .expect("Unwrapping pool in setup view (moving to auth view)"),
                        ),
                    );
                }
            }
            AppViews::Main(view) => view.update(
                ctx,
                &self.runtime,
                Arc::clone(self.pool.as_ref().expect("Unwrapping pool in main view")),
            ),
        }
    }
}
