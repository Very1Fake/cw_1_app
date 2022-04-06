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
    pub const NAME: &'static str = "Service";

    pub const CREATE: &'static str = r#"CREATE TABLE "Service" (
    uuid uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    name text NOT NULL UNIQUE,
    description text,
    price money NOT NULL,
    meta metatime NOT NULL DEFAULT (current_timestamp, current_timestamp)
);"#;

    pub const DROP: &'static str = r#"DROP TABLE "Service";"#;

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
