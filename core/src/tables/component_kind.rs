use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgArguments, query, query::Query, Postgres};
use uuid::Uuid;

use crate::traits::Insertable;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ComponentKind {
    pub uuid: Uuid,
    pub name: String,
    pub details: Option<String>,
}

impl ComponentKind {
    pub const NAME: &'static str = "ComponentKind";

    pub const CREATE: &'static str = r#"CREATE TABLE "ComponentKind" (
    uuid uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    name text NOT NULL,
    details text
)"#;

    pub const DROP: &'static str = r#"DROP TABLE "ComponentKind";"#;

    pub const SAMPLES: [(&'static str, Option<&'static str>, f64); 5] = [
        ("Battery", None, 5000.0),
        ("Screen Display", None, 10000.0),
        ("RAM", None, 20000.0),
        ("Memory", None, 18500.0),
        ("Screen Glass", None, 12500.0),
    ];

    pub const fn new(uuid: Uuid, name: String, details: Option<String>) -> Self {
        Self {
            uuid,
            name,
            details,
        }
    }

    pub fn new_auto(name: String, details: Option<String>) -> Self {
        Self::new(Uuid::new_v4(), name, details)
    }
}

impl Insertable for ComponentKind {
    fn insert(&self) -> Query<'static, Postgres, PgArguments> {
        query(r#"INSERT INTO "ComponentKind" (uuid, name, details) VALUES ($1, $2, $3);"#)
            .bind(self.uuid)
            .bind(self.name.clone())
            .bind(self.details.clone())
    }
}
