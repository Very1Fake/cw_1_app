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
    pub const NAME: &'static str = "SupplyContract";

    pub const CREATE: &'static str = r#"CREATE TABLE "SupplyContract" (
    uuid uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    supplier uuid NOT NULL REFERENCES "Supplier" ON DELETE restrict ON UPDATE cascade,
    status "ContractStatus" NOT NULL DEFAULT 'Review',
    signed timestamp,
    meta metatime NOT NULL DEFAULT (current_timestamp, current_timestamp)
);"#;

    pub const DROP: &'static str = r#"DROP TABLE "SupplyContract";"#;

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
