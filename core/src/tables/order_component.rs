use uuid::Uuid;

/// Represents relation table between [`Order`](`super::order::Order`) and [`Warehouse`](`super::warehouse::Warehouse`)
#[derive(Debug)]
pub struct OrderComponent {
    /// Foreign key references [`Order`](`super::order::Order`)
    pub order: Uuid,
    /// Foreign key references [`Warehouse`](`super::warehouse::Warehouse`)
    pub item: Uuid,
    pub amount: i32,
    pub price: f64,
}

impl OrderComponent {
    pub const fn new(order: Uuid, item: Uuid, amount: i32, price: f64) -> Self {
        Self {
            order,
            item,
            amount,
            price,
        }
    }
}
