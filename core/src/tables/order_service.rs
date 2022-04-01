use uuid::Uuid;

/// Represents relation table between [`Order`](`super::order::Order`) and [`Service`](`super::service::Service`)
#[derive(Debug)]
pub struct OrderService {
    /// Foreign key references [`Order`](`super::order::Order`)
    pub order: Uuid,
    /// Foreign key references [`Service`](`super::service::Service`)
    pub service: Uuid,
    pub price: f64,
}

impl OrderService {
    pub const fn new(order: Uuid, service: Uuid, price: f64) -> Self {
        Self {
            order,
            service,
            price,
        }
    }
}
