use crate::model::user::User;

use self::auth::AuthView;
use self::setup::SetupView;

pub mod auth;
pub mod setup;

pub enum AppViews {
    Auth(AuthView),
    Setup(SetupView),
    Main { user: User },
}

impl AppViews {
    pub fn setup() -> Self {
        Self::Setup(SetupView::new())
    }

    pub fn auth() -> Self {
        Self::Auth(AuthView::new())
    }
}
