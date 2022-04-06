use std::time::Instant;

use uuid::Uuid;

use crate::types::{metatime::MetaTime, supply_status::SupplyStatus};

#[derive(Debug)]
pub struct Supply {
    pub uuid: Uuid,
    /// Foreign key references [`SupplyContract`](`super::supply_contract::SupplyContract`)
    pub contract: Uuid,
    /// Foreign key references [`Staff`](`super::staff::Staff`)
    pub staff: Uuid,
    pub status: SupplyStatus,
    pub signed: Option<Instant>,
    pub meta: MetaTime,
}

impl Supply {
    pub const NAME: &'static str = "Supply";

    pub const CREATE: &'static str = r#"CREATE TABLE "Supply" (
    uuid uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    contract uuid NOT NULL REFERENCES "SupplyContract" ON DELETE restrict ON UPDATE cascade,
    staff uuid NOT NULL REFERENCES "Staff" ON DELETE restrict ON UPDATE cascade,
    status "SupplyStatus" NOT NULL DEFAULT 'Review',
    signed timestamp,
    meta metatime NOT NULL DEFAULT (current_timestamp, current_timestamp)
);"#;

    pub const DROP: &'static str = r#"DROP TABLE "Supply";"#;

    pub const fn new(
        uuid: Uuid,
        contract: Uuid,
        staff: Uuid,
        status: SupplyStatus,
        signed: Option<Instant>,
        meta: MetaTime,
    ) -> Self {
        Self {
            uuid,
            contract,
            staff,
            status,
            signed,
            meta,
        }
    }
}
