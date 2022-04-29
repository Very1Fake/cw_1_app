use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgQueryResult, query, types::BigDecimal, Error, PgPool};
use uuid::Uuid;

use crate::types::MetaTime;

#[derive(Serialize, Deserialize, Debug)]
pub struct Warehouse {
    pub uuid: Uuid,
    /// Foreign key references [`Component`](`super::component::Component`)
    pub component: Uuid,
    /// Foreign key references [`Supplier`](`super::supplier::Supplier`)
    pub supplier: Uuid,
    pub price: BigDecimal,
    pub amount: i32,
    pub meta: MetaTime,
}

impl Warehouse {
    pub const NAME: &'static str = "Warehouse";

    pub const CREATE: &'static str = r#"CREATE TABLE "Warehouse" (
    uuid uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    component uuid NOT NULL REFERENCES "Component" ON DELETE restrict ON UPDATE cascade,
    supplier uuid NOT NULL REFERENCES "Supplier" ON DELETE restrict ON UPDATE cascade,
    price money NOT NULL,
    amount int NOT NULL DEFAULT 0,
    meta metatime NOT NULL DEFAULT (now(), now()),
    UNIQUE(component, supplier)
);"#;

    pub const DROP: &'static str = r#"DROP TABLE "Warehouse";"#;

    pub const fn new(
        uuid: Uuid,
        component: Uuid,
        supplier: Uuid,
        price: BigDecimal,
        amount: i32,
        meta: MetaTime,
    ) -> Self {
        Self {
            uuid,
            component,
            supplier,
            price,
            amount,
            meta,
        }
    }

    pub fn new_auto(component: Uuid, supplier: Uuid, price: BigDecimal, amount: i32) -> Self {
        Self::new(
            Uuid::new_v4(),
            component,
            supplier,
            price,
            amount,
            MetaTime::default(),
        )
    }

    pub async fn insert(&self, pool: &PgPool) -> Result<PgQueryResult, Error> {
        query(
            r#"INSERT INTO "Warehouse" (uuid, component, supplier, price, amount)
VALUES ($1, $2, $3, $4, $5);"#,
        )
        .bind(self.uuid)
        .bind(self.component)
        .bind(self.supplier)
        .bind(self.price.clone())
        .bind(self.amount)
        .execute(pool)
        .await
    }
}
