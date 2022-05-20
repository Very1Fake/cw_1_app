use crate::model::user::User;

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
    pub fn setup() -> Self {
        Self::Setup(SetupView::new())
    }

    pub fn auth() -> Self {
        Self::Auth(AuthView::new())
    }

    pub fn main(user: User) -> Self {
        Self::Main(MainView::new(user))
    }
}
