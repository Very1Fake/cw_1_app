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
    pub fn setup(config: &Config, runtime: &Runtime) -> Self {
        Self::Setup(SetupView::new_with_config(config, runtime))
    }

    pub fn auth(config: &Config, runtime: &Runtime, pool: Pool) -> Self {
        Self::Auth(AuthView::new_with_config(config, runtime, pool))
    }

    pub fn main(user: User) -> Self {
        Self::Main(MainView::new(user))
    }
}
