use sqlx::types::Uuid;

use crate::types::{color::Color, metatime::MetaTime};

#[derive(Debug)]
pub struct Phone {
    pub uuid: Uuid,
    /// Foreign key references [`Person`](`super::person::Person`)
    pub name: String,
    pub imei: String,
    pub wifi: String,
    pub bluetooth: String, // FIX
    /// Foreign key references [`PhoneModel`](`super::phone_model::PhoneModel`)
    pub model: Uuid,
    pub color: Color,
    pub meta: MetaTime,
}

impl Phone {
    pub const NAME: &'static str = "Phone";

    pub const CREATE: &'static str = r#"CREATE TABLE "Phone" (
    uuid uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    person uuid NOT NULL REFERENCES "Person" ON DELETE restrict ON UPDATE cascade,
    imei text NOT NULL CHECK (length(imei) <= 17),
    wifi macaddr,
    bluetooth macaddr,
    model uuid NOT NULL REFERENCES "PhoneModel" ON DELETE restrict ON UPDATE cascade,
    color color NOT NULL,
    meta metatime NOT NULL DEFAULT (current_timestamp, current_timestamp)
);"#;

    pub const DROP: &'static str = r#"DROP TABLE "Phone";"#;

    pub const fn new(
        uuid: Uuid,
        name: String,
        imei: String,
        wifi: String,
        bluetooth: String,
        model: Uuid,
        color: Color,
        meta: MetaTime,
    ) -> Self {
        Self {
            uuid,
            name,
            imei,
            wifi,
            bluetooth,
            model,
            color,
            meta,
        }
    }
}
