use sqlx::{postgres::PgQueryResult, query, Error, PgPool};
use uuid::Uuid;

use crate::types::metatime::MetaTime;

#[derive(Debug)]
pub struct Service {
    pub uuid: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub meta: MetaTime,
}

impl Service {
    pub const NAME: &'static str = "Service";

    pub const CREATE: &'static str = r#"CREATE TABLE "Service" (
    uuid uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    name text NOT NULL UNIQUE,
    description text,
    meta metatime NOT NULL DEFAULT (now(), now())
);"#;

    pub const DROP: &'static str = r#"DROP TABLE "Service";"#;

    pub const fn new(
        uuid: Uuid,
        name: String,
        description: Option<String>,
        meta: MetaTime,
    ) -> Self {
        Self {
            uuid,
            name,
            description,
            meta,
        }
    }

    pub fn new_auto(name: String, description: Option<String>) -> Self {
        Self::new(Uuid::new_v4(), name, description, MetaTime::default())
    }

    pub async fn insert(&self, pool: &PgPool) -> Result<PgQueryResult, Error> {
        query(
            r#"INSERT INTO "Service" (uuid, name, description)
    VALUES ($1, $2, $3);"#,
        )
        .bind(self.uuid)
        .bind(self.name.clone())
        .bind(self.description.clone())
        .execute(pool)
        .await
    }
}
