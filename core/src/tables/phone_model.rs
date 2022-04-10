use sqlx::{postgres::PgQueryResult, query, Error, PgPool};
use uuid::Uuid;

#[derive(Debug)]
pub struct PhoneModel {
    pub uuid: Uuid,
    pub name: String,
    pub description: Option<String>,
    /// Foreign key references [`Manufacturer`](super::manufacturer::Manufacturer)
    pub manufacturer: Uuid,
}

impl PhoneModel {
    pub const NAME: &'static str = "PhoneModel";

    pub const CREATE: &'static str = r#"CREATE TABLE "PhoneModel" (
    uuid uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    name text NOT NULL UNIQUE,
    description text,
    manufacturer uuid NOT NULL REFERENCES "Manufacturer" ON DELETE restrict ON UPDATE cascade
);"#;

    pub const DROP: &'static str = r#"DROP TABLE "PhoneModel";"#;

    pub const fn new(
        uuid: Uuid,
        name: String,
        description: Option<String>,
        manufacturer: Uuid,
    ) -> Self {
        Self {
            uuid,
            name,
            description,
            manufacturer,
        }
    }

    pub fn new_auto(name: String, description: Option<String>, manufacturer: Uuid) -> Self {
        Self::new(Uuid::new_v4(), name, description, manufacturer)
    }

    pub async fn insert(&self, pool: &PgPool) -> Result<PgQueryResult, Error> {
        query(
            r#"INSERT INTO "PhoneModel" (uuid, name, description, manufacturer) 
    VALUES ($1, $2, $3, $4);"#,
        )
        .bind(self.uuid)
        .bind(self.name.clone())
        .bind(self.description.clone())
        .bind(self.manufacturer)
        .execute(pool)
        .await
    }
}
