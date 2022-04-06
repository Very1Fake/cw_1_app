use uuid::Uuid;

use crate::types::{metatime::MetaTime, order_status::OrderStatus};

#[derive(Debug)]
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
    client uuid NOT NULL REFERENCES "Person" ON DELETE restrict ON UPDATE cascade,
    phone uuid NOT NULL REFERENCES "Phone" ON DELETE restrict ON UPDATE cascade,
    serviceman uuid NOT NULL REFERENCES "Staff" ON DELETE restrict ON UPDATE cascade,
    shopman uuid NOT NULL REFERENCES "Staff" ON DELETE restrict ON UPDATE cascade,
    status "OrderStatus" NOT NULL DEFAULT 'Processing',
    meta metatime DEFAULT (current_timestamp, current_timestamp)
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
}
