use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgArguments, query, query::Query, query_as, FromRow, Postgres};
use uuid::Uuid;

use crate::{traits::Insertable, PgQuery, PgQueryAs};

#[derive(FromRow, Serialize, Deserialize, Clone, Debug)]
pub struct Manufacturer {
    pub uuid: Uuid,
    pub name: String,
    pub country: String,
}

impl Manufacturer {
    pub const NAME: &'static str = "Manufacturer";

    pub const CREATE: &'static str = r#"CREATE TABLE "Manufacturer" (
    uuid uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    name text NOT NULL,
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

    pub fn delete_by_uuid(uuid: Uuid) -> PgQuery {
        query(r#"DELETE FROM "Manufacturer" WHERE uuid = $1"#).bind(uuid)
    }

    pub fn get_all() -> PgQueryAs<Self> {
        query_as(r#"SELECT * FROM "Manufacturer""#)
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
