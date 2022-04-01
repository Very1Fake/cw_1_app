use std::time::Instant;

use uuid::Uuid;

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
    pub const fn new(item: Uuid, supply: Uuid, amount: i32, created: Instant) -> Self {
        Self {
            item,
            supply,
            amount,
            created,
        }
    }
}
