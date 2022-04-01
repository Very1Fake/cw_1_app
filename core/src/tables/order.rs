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
