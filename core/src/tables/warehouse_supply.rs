use std::time::Instant;

use sqlx::types::Uuid;

/// Represents relation table between [`Warehouse`](`super::warehouse::Warehouse`) and [`Supply`](`super::supply::Supply`)
#[derive(Debug)]
pub struct WarehouseSupply {
    /// Foreign key references [`Warehouse`](`super::warehouse::Warehouse`)
    pub item: Uuid,
    /// Foreign key references [`Supply`](`super::supply::Supply`)
    pub supply: Uuid,
    pub amount: i32,
    pub created: Instant,
}

impl WarehouseSupply {
    pub const NAME: &'static str = "WarehouseSupply";

    pub const CREATE: &'static str = r#"CREATE TABLE "WarehouseSupply" (
    item uuid NOT NULL REFERENCES "Warehouse" ON DELETE restrict ON UPDATE cascade,
    supply uuid NOT NULL REFERENCES "Supply" ON DELETE restrict ON UPDATE cascade,
    amount int NOT NULL,
    created timestamp NOT NULL DEFAULT current_timestamp,
    PRIMARY KEY(item, supply)
);"#;

    pub const DROP: &'static str = r#"DROP TABLE "WarehouseSupply";"#;

    pub const fn new(item: Uuid, supply: Uuid, amount: i32, created: Instant) -> Self {
        Self {
            item,
            supply,
            amount,
            created,
        }
    }
}
