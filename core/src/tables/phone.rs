use mac_address::MacAddress;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgQueryResult, query, Error, PgPool};
use uuid::Uuid;

use crate::types::{color::Color, metatime::MetaTime};

#[derive(Serialize, Deserialize, Debug)]
pub struct Phone {
    pub uuid: Uuid,
    /// Foreign key references [`Person`](`super::person::Person`)
    pub person: Uuid,
    pub imei: String,
    pub wifi: MacAddress,
    pub bluetooth: MacAddress,
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
    meta metatime NOT NULL DEFAULT (now(), now())
);"#;

    pub const DROP: &'static str = r#"DROP TABLE "Phone";"#;

    pub const fn new(
        uuid: Uuid,
        person: Uuid,
        imei: String,
        wifi: MacAddress,
        bluetooth: MacAddress,
        model: Uuid,
        color: Color,
        meta: MetaTime,
    ) -> Self {
        Self {
            uuid,
            person,
            imei,
            wifi,
            bluetooth,
            model,
            color,
            meta,
        }
    }

    pub fn new_auto(
        person: Uuid,
        imei: String,
        wifi: MacAddress,
        bluetooth: MacAddress,
        model: Uuid,
        color: Color,
    ) -> Self {
        Self::new(
            Uuid::new_v4(),
            person,
            imei,
            wifi,
            bluetooth,
            model,
            color,
            MetaTime::default(),
        )
    }

    pub async fn insert(&self, pool: &PgPool) -> Result<PgQueryResult, Error> {
        query(
            r#"INSERT INTO "Phone" (uuid, person, imei, wifi, bluetooth, model, color) 
VALUES ($1, $2, $3, $4, $5, $6, $7);"#,
        )
        .bind(self.uuid)
        .bind(self.person)
        .bind(self.imei.clone())
        .bind(self.wifi)
        .bind(self.bluetooth)
        .bind(self.model)
        .bind(self.color)
        .execute(pool)
        .await
    }
}
