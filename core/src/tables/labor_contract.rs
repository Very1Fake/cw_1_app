use std::time::Instant;

use uuid::Uuid;

use crate::types::{contract_status::ContractStatus, metatime::MetaTime};

#[derive(Debug)]
pub struct LaborContract {
    pub uuid: Uuid,
    /// Foreign key references [`Person`](`super::person::Person`)
    pub person: Uuid,
    pub passport: String,
    pub status: ContractStatus,
    pub signed: Option<Instant>,
    pub meta: MetaTime,
}

impl LaborContract {
    pub const NAME: &'static str = "LaborContract";

    pub const CREATE: &'static str = r#"CREATE TABLE "LaborContract" (
    uuid uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    person uuid NOT NULL REFERENCES "Person" ON DELETE restrict ON UPDATE cascade,
    passport char(10) NOT NULL UNIQUE,
    status "ContractStatus" NOT NULL DEFAULT 'Review',
    signed timestamp,
    meta metatime NOT NULL DEFAULT (current_timestamp, current_timestamp)
);"#;

    pub const DROP: &'static str = r#"DROP TABLE "LaborContract";"#;

    pub const fn new(
        uuid: Uuid,
        person: Uuid,
        passport: String,
        status: ContractStatus,
        signed: Option<Instant>,
        meta: MetaTime,
    ) -> Self {
        Self {
            uuid,
            person,
            passport,
            status,
            signed,
            meta,
        }
    }
}
