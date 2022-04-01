use std::time::Instant;

use uuid::Uuid;

use crate::types::{contract_status::ContractStatus, metatime::MetaTime};

#[derive(Debug)]
pub struct LaborContract {
    pub uuid: Uuid,
    /// Foreign key references [`Person`](`super::person::Person`)
    pub person: Uuid,
    pub status: ContractStatus,
    pub signed: Option<Instant>,
    pub meta: MetaTime,
}

impl LaborContract {
    pub const fn new(
        uuid: Uuid,
        person: Uuid,
        status: ContractStatus,
        signed: Option<Instant>,
        meta: MetaTime,
    ) -> Self {
        Self {
            uuid,
            person,
            status,
            signed,
            meta,
        }
    }
}
