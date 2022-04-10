use sqlx::{postgres::PgQueryResult, query, types::BigDecimal, Error, PgPool};
use uuid::Uuid;

use crate::types::MetaTime;

/// Represents relation table between [`Service`](`super::service::Service`) and [`PhoneModel`](`super::phone_model::PhoneModel`)
#[derive(Debug)]
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

    pub async fn insert(&self, pool: &PgPool) -> Result<PgQueryResult, Error> {
        query(r#"INSERT INTO "ServicePhoneModel" VALUES ($1, $2, $3);"#)
            .bind(self.service)
            .bind(self.phone_model)
            .bind(self.price.clone())
            .execute(pool)
            .await
    }
}
