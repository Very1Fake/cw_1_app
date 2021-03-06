use mac_address::MacAddress;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgArguments, query, query::Query, FromRow, Postgres};
use uuid::Uuid;

use crate::{
    traits::Insertable,
    types::{color::Color, metatime::MetaTime},
    PgQuery,
};

#[derive(FromRow, Serialize, Deserialize, Clone, Debug)]
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
    person uuid NOT NULL REFERENCES "Person" ON DELETE no action ON UPDATE cascade,
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

    pub fn delete_by_uuid(uuid: Uuid) -> PgQuery {
        query(r#"DELETE FROM "Service" WHERE uuid = $1"#).bind(uuid)
    }
}
impl Insertable for Phone {
    fn insert(&self) -> Query<'static, Postgres, PgArguments> {
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
    }
}
