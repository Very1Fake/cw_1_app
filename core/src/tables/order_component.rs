use sqlx::{postgres::PgQueryResult, query, types::BigDecimal, Error, PgPool};
use uuid::Uuid;

/// Represents relation table between [`Order`](`super::order::Order`) and [`Warehouse`](`super::warehouse::Warehouse`)
#[derive(Debug)]
pub struct OrderWarehouse {
    /// Foreign key references [`Order`](`super::order::Order`)
    pub order: Uuid,
    /// Foreign key references [`Warehouse`](`super::warehouse::Warehouse`)
    pub item: Uuid,
    pub amount: i32,
    pub price: BigDecimal,
}

impl OrderWarehouse {
    pub const NAME: &'static str = "OrderWarehouse";

    pub const CREATE: &'static str = r#"CREATE TABLE "OrderWarehouse" (
    "order" uuid NOT NULL REFERENCES "Order" ON DELETE cascade ON UPDATE cascade,
    item uuid NOT NULL REFERENCES "Warehouse" ON DELETE restrict ON UPDATE cascade,
    amount int NOT NULL DEFAULT 1,
    price money NOT NULL
);"#;

    pub const DROP: &'static str = r#"DROP TABLE "OrderWarehouse";"#;

    pub const fn new(order: Uuid, item: Uuid, amount: i32, price: BigDecimal) -> Self {
        Self {
            order,
            item,
            amount,
            price,
        }
    }

    pub async fn insert(&self, pool: &PgPool) -> Result<PgQueryResult, Error> {
        query(r#"INSERT INTO "OrderWarehouse" VALUES ($1, $2, $3, $4);"#)
            .bind(self.order)
            .bind(self.item)
            .bind(self.amount)
            .bind(self.price.clone())
            .execute(pool)
            .await
    }
}
