use tokio::runtime::Runtime;

use crate::model::config::Config;
use crate::model::user::User;
use crate::utils::Pool;

use self::auth::AuthView;
use self::main::MainView;
use self::setup::SetupView;

pub mod auth;
pub mod main;
pub mod setup;

pub enum AppViews {
    Auth(AuthView),
    Setup(SetupView),
    Main(MainView),
}

impl AppViews {
    pub fn setup(config: &Config) -> Self {
        Self::Setup(SetupView::from_config(config))
    }

    pub fn auth(config: &Config) -> Self {
        Self::Auth(AuthView::from_config(config))
    }

    pub fn setup_reactive(config: &Config, runtime: &Runtime) -> Self {
        Self::Setup(SetupView::reactive(config, runtime))
    }

    pub fn auth_reactive(config: &Config, runtime: &Runtime, pool: Pool) -> Self {
        Self::Auth(AuthView::reactive(config, runtime, pool))
    }

    pub fn main(user: User) -> Self {
        Self::Main(MainView::new(user))
    }
}

pub enum ViewResponse<T> {
    Remain,
    Next((T, bool)), // TODO: Make reactive move
}

impl<T> ViewResponse<T> {
    pub fn next(payload: T, reactive: bool) -> Self {
        Self::Next((payload, reactive))
    }
}
