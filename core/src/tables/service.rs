use uuid::Uuid;

use crate::types::metatime::MetaTime;

#[derive(Debug)]
pub struct Service {
    pub uuid: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub price: f64,
    pub meta: MetaTime,
}

impl Service {
    pub const fn new(
        uuid: Uuid,
        name: String,
        description: Option<String>,
        price: f64,
        meta: MetaTime,
    ) -> Self {
        Self {
            uuid,
            name,
            description,
            price,
            meta,
        }
    }
}
