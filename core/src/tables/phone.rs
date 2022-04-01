use uuid::Uuid;

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
