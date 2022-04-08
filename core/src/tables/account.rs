use sqlx::types::Uuid;

use crate::types::{account_role::AccountRole, account_status::AccountStatus, metatime::MetaTime};

#[derive(Debug)]
pub struct Account {
    pub uuid: Uuid,
    /// Foreign key references [`Staff`](`super::staff::Staff`)
    pub staff: Uuid,
    pub login: String,
    pub password: String, // TODO: Impl password hash object instead
    pub role: AccountRole,
    pub status: AccountStatus,
    pub meta: MetaTime,
}

impl Account {
    pub const NAME: &'static str = "Account";

    pub const CREATE: &'static str = r#"CREATE TABLE "Account" (
    uuid uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    staff uuid NOT NULL REFERENCES "Staff" ON DELETE restrict ON UPDATE cascade,
    login text NOT NULL UNIQUE CHECK (length(login) <= 24),
    password text NOT NULL CHECK (length(password) <= 96),
    role "AccountRole" NOT NULL,
    status "AccountStatus" NOT NULL DEFAULT 'Inactive',
    meta metatime NOT NULL DEFAULT (current_timestamp, current_timestamp)
);"#;

    pub const DROP: &'static str = r#"DROP TABLE "Account";"#;

    pub const fn new(
        uuid: Uuid,
        staff: Uuid,
        login: String,
        password: String,
        role: AccountRole,
        status: AccountStatus,
        meta: MetaTime,
    ) -> Self {
        Self {
            uuid,
            staff,
            login,
            password,
            role,
            status,
            meta,
        }
    }
}
