use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgArguments, query, query::Query, types::BigDecimal, Postgres};
use uuid::Uuid;

use crate::{traits::Insertable, types::MetaTime};

/// Represents relation table between [`Service`](`super::service::Service`) and [`PhoneModel`](`super::phone_model::PhoneModel`)
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ServicePhoneModel {
    /// Foreign key references [`Service`](`super::service::Service`)
    pub service: Uuid,
    /// Foreign key references [`PhoneModel`](`super::phone_model::PhoneModel`)
    pub phone_model: Uuid,
    /// Recommended price
    pub price: BigDecimal,
    pub meta: MetaTime,
}

impl ServicePhoneModel {
    pub const NAME: &'static str = "ServicePhoneModel";

    pub const CREATE: &'static str = r#"CREATE TABLE "ServicePhoneModel" (
    service uuid NOT NULL REFERENCES "Service" ON DELETE restrict ON UPDATE cascade,
    phone_model uuid NOT NULL REFERENCES "PhoneModel" ON DELETE restrict ON UPDATE cascade,
    price money,
    meta metatime NOT NULL DEFAULT (now(), now()),
    PRIMARY KEY (service, phone_model)
);"#;

    pub const DROP: &'static str = r#"DROP TABLE "ServicePhoneModel";"#;

    pub const fn new(service: Uuid, phone_model: Uuid, price: BigDecimal, meta: MetaTime) -> Self {
        Self {
            service,
            phone_model,
            price,
            meta,
        }
    }

    pub fn new_auto(service: Uuid, phone_model: Uuid, price: BigDecimal) -> Self {
        Self::new(service, phone_model, price, MetaTime::default())
    }
}
impl Insertable for ServicePhoneModel {
    fn insert(&self) -> Query<'static, Postgres, PgArguments> {
        query(r#"INSERT INTO "ServicePhoneModel" VALUES ($1, $2, $3);"#)
            .bind(self.service)
            .bind(self.phone_model)
            .bind(self.price.clone())
    }
}
