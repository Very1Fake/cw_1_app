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
