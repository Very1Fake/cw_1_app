use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgArguments, query, query::Query, Postgres};
use uuid::Uuid;

use crate::traits::Insertable;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Manufacturer {
    pub uuid: Uuid,
    pub name: String,
    pub country: String,
}

impl Manufacturer {
    pub const NAME: &'static str = "Manufacturer";

    pub const CREATE: &'static str = r#"CREATE TABLE "Manufacturer" (
    uuid uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    name text,
    country char(2) NOT NULL
);"#;

    pub const DROP: &'static str = r#"DROP TABLE "Manufacturer";"#;

    pub const SAMPLES: [(&'static str, &'static str); 4] = [
        ("Apple", "US"),
        ("Samsung", "KR"),
        ("Huawei", "CN"),
        ("FoxCon", "CN"),
    ];

    pub const fn new(uuid: Uuid, name: String, country: String) -> Self {
        Self {
            uuid,
            name,
            country,
        }
    }

    pub fn new_auto(name: String, country: String) -> Self {
        Self::new(Uuid::new_v4(), name, country)
    }
}
impl Insertable for Manufacturer {
    fn insert(&self) -> Query<'static, Postgres, PgArguments> {
        query(r#"INSERT INTO "Manufacturer" (uuid, name, country) VALUES ($1, $2, $3);"#)
            .bind(self.uuid)
            .bind(self.name.clone())
            .bind(self.country.clone())
    }
}
