use sqlx::types::Uuid;

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
    pub const NAME: &'static str = "OrderService";

    pub const CREATE: &'static str = r#"CREATE TABLE "OrderService" (
    "order" uuid NOT NULL REFERENCES "Order" ON DELETE cascade ON UPDATE cascade,
    service uuid NOT NULL REFERENCES "Service" ON DELETE restrict ON UPDATE cascade,
    price money NOT NULL
);"#;

    pub const DROP: &'static str = r#"DROP TABLE "OrderService";"#;

    pub const fn new(order: Uuid, service: Uuid, price: f64) -> Self {
        Self {
            order,
            service,
            price,
        }
    }
}
