use std::time::Instant;

use uuid::Uuid;

use crate::types::{contract_status::ContractStatus, metatime::MetaTime};

#[derive(Debug)]
pub struct SupplyContract {
    pub uuid: Uuid,
    /// Foreign key references [`Supplier`](super::supplier::Supplier)
    pub supplier: Uuid,
    pub status: ContractStatus,
    pub signed: Instant,
    pub meta: MetaTime,
}

impl SupplyContract {
    pub const fn new(
        uuid: Uuid,
        supplier: Uuid,
        status: ContractStatus,
        signed: Instant,
        meta: MetaTime,
    ) -> Self {
        Self {
            uuid,
            supplier,
            status,
            signed,
            meta,
        }
    }
}
