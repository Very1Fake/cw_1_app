use serde::{Deserialize, Serialize};
use sqlx::{
    postgres::{types::PgMoney, PgArguments},
    query,
    query::Query,
    FromRow, Postgres,
};
use uuid::Uuid;

use crate::traits::Insertable;

/// Represents relation table between [`Order`](`super::order::Order`) and [`Service`](`super::service::Service`)
#[derive(FromRow, Serialize, Deserialize, Clone, Debug)]
pub struct OrderService {
    /// Foreign key references [`Order`](`super::order::Order`)
    pub order: Uuid,
    /// Foreign key references [`Service`](`super::service::Service`)
    pub service: Uuid,
    #[serde(
        deserialize_with = "crate::utils::deserialize_pg_money",
        serialize_with = "crate::utils::serialize_pg_money"
    )]
    pub price: PgMoney,
}

impl OrderService {
    pub const NAME: &'static str = "OrderService";

    pub const CREATE: &'static str = r#"CREATE TABLE "OrderService" (
    "order" uuid NOT NULL REFERENCES "Order" ON DELETE cascade ON UPDATE cascade,
    service uuid NOT NULL REFERENCES "Service" ON DELETE restrict ON UPDATE cascade,
    price money NOT NULL
);"#;

    pub const DROP: &'static str = r#"DROP TABLE "OrderService";"#;

    pub fn new(order: Uuid, service: Uuid, price: PgMoney) -> Self {
        Self {
            order,
            service,
            price,
        }
    }
}

impl Insertable for OrderService {
    fn insert(&self) -> Query<'static, Postgres, PgArguments> {
        query(r#"INSERT INTO "OrderService" VALUES ($1, $2, $3);"#)
            .bind(self.order)
            .bind(self.service)
            .bind(self.price)
    }
}
