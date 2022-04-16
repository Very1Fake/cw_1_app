use sqlx::{postgres::PgQueryResult, query, Error, PgPool};
use uuid::Uuid;

#[derive(Debug)]
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

    pub const SAMPLES: [(&'static str, &'static str); 3] =
        [("Apple", "US"), ("Samsung", "KR"), ("Huawei", "CN")];

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

    pub async fn insert(&self, pool: &PgPool) -> Result<PgQueryResult, Error> {
        query(r#"INSERT INTO "Manufacturer" (uuid, name, country) VALUES ($1, $2, $3);"#)
            .bind(self.uuid)
            .bind(self.name.clone())
            .bind(self.country.clone())
            .execute(pool)
            .await
    }
}
