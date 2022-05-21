use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgArguments, query, query::Query, FromRow, Postgres};
use uuid::Uuid;

use crate::{
    traits::Insertable,
    types::{metatime::MetaTime, order_status::OrderStatus},
};

#[derive(FromRow, Serialize, Deserialize, Clone, Debug)]
pub struct Order {
    pub uuid: Uuid,
    /// Foreign key references [`Person`](`super::person::Person`)
    pub client: Uuid,
    /// Foreign key references [`Phone`](`super::phone::Phone`)
    pub phone: Uuid,
    /// Foreign key references [`Staff`](`super::staff::Staff`)
    pub serviceman: Uuid,
    /// Foreign key references [`Staff`](`super::staff::Staff`)
    pub shopman: Uuid,
    pub status: OrderStatus,
    pub meta: MetaTime,
}

impl Order {
    pub const NAME: &'static str = "Order";

    pub const CREATE: &'static str = r#"CREATE TABLE "Order" (
    uuid uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    client uuid NOT NULL REFERENCES "Person" ON DELETE no action ON UPDATE cascade,
    phone uuid NOT NULL REFERENCES "Phone" ON DELETE restrict ON UPDATE cascade,
    serviceman uuid NOT NULL REFERENCES "Staff" ON DELETE restrict ON UPDATE cascade,
    shopman uuid NOT NULL REFERENCES "Staff" ON DELETE restrict ON UPDATE cascade,
    status "OrderStatus" NOT NULL DEFAULT 'Processing',
    meta metatime DEFAULT (now(), now())
);"#;

    pub const DROP: &'static str = r#"DROP TABLE "Order";"#;

    pub const fn new(
        uuid: Uuid,
        client: Uuid,
        phone: Uuid,
        serviceman: Uuid,
        shopman: Uuid,
        status: OrderStatus,
        meta: MetaTime,
    ) -> Self {
        Self {
            uuid,
            client,
            phone,
            serviceman,
            shopman,
            status,
            meta,
        }
    }

    pub fn new_auto(
        client: Uuid,
        phone: Uuid,
        serviceman: Uuid,
        shopman: Uuid,
        status: OrderStatus,
    ) -> Self {
        Self::new(
            Uuid::new_v4(),
            client,
            phone,
            serviceman,
            shopman,
            status,
            MetaTime::default(),
        )
    }
}
impl Insertable for Order {
    fn insert(&self) -> Query<'static, Postgres, PgArguments> {
        query(
            r#"INSERT INTO "Order" (uuid, client, phone, serviceman, shopman, status) 
VALUES ($1, $2, $3, $4, $5, $6);
          "#,
        )
        .bind(self.uuid)
        .bind(self.client)
        .bind(self.phone)
        .bind(self.serviceman)
        .bind(self.shopman)
        .bind(self.status)
    }
}
