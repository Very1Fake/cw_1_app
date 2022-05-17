use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgArguments, query, query::Query, Postgres};
use uuid::Uuid;

use crate::{traits::Insertable, types::metatime::MetaTime};

#[derive(Serialize, Deserialize, Clone, Debug)]
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

    pub const SAMPLES: [(&'static str, Option<&'static str>, f64, &'static str); 5] = [
        ("Battery replacement", None, 10000.0, "Battery"),
        (
            "Screen display replacement",
            None,
            15000.0,
            "Screen Display",
        ),
        (
            "RAM Fix",
            Some("Replace malfunctioning RAM bank"),
            25000.0,
            "RAM",
        ),
        (
            "Memory Fix",
            Some("Replace malfunctioning memory bank"),
            20000.0,
            "Memory",
        ),
        ("Screen glass replacement", None, 12500.0, "Screen Glass"),
    ];

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
}

impl Insertable for Service {
    fn insert(&self) -> Query<'static, Postgres, PgArguments> {
        query(
            r#"INSERT INTO "Service" (uuid, name, description)
    VALUES ($1, $2, $3);"#,
        )
        .bind(self.uuid)
        .bind(self.name.clone())
        .bind(self.description.clone())
    }
}
