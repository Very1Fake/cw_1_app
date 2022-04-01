use uuid::Uuid;

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
