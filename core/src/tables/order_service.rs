use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgQueryResult, query, types::BigDecimal, Error, PgPool};
use uuid::Uuid;

/// Represents relation table between [`Order`](`super::order::Order`) and [`Service`](`super::service::Service`)
#[derive(Serialize, Deserialize, Debug)]
pub struct OrderService {
    /// Foreign key references [`Order`](`super::order::Order`)
    pub order: Uuid,
    /// Foreign key references [`Service`](`super::service::Service`)
    pub service: Uuid,
    pub price: BigDecimal,
}

impl OrderService {
    pub const NAME: &'static str = "OrderService";

    pub const CREATE: &'static str = r#"CREATE TABLE "OrderService" (
    "order" uuid NOT NULL REFERENCES "Order" ON DELETE cascade ON UPDATE cascade,
    service uuid NOT NULL REFERENCES "Service" ON DELETE restrict ON UPDATE cascade,
    price money NOT NULL
);"#;

    pub const DROP: &'static str = r#"DROP TABLE "OrderService";"#;

    pub const fn new(order: Uuid, service: Uuid, price: BigDecimal) -> Self {
        Self {
            order,
            service,
            price,
        }
    }

    pub async fn insert(&self, pool: &PgPool) -> Result<PgQueryResult, Error> {
        query(r#"INSERT INTO "OrderService" VALUES ($1, $2, $3);"#)
            .bind(self.order)
            .bind(self.service)
            .bind(self.price.clone())
            .execute(pool)
            .await
    }
}
