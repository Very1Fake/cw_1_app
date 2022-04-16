use sqlx::{postgres::PgQueryResult, query, Error, PgPool};
use uuid::Uuid;

#[derive(Debug)]
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

    pub const SAMPLES: [(&'static str, Option<&'static str>); 16] = [
        ("Battery Li-Pol", Some("Lithium polymer battery")),
        ("Battery Li-Ion", Some("Lithium ion battery")),
        ("Battery Ni-CD", Some("Nickel cadmium battery")),
        ("Battery Ni-MH", Some("Nickel metal hydride battery")),
        ("Display (IPS)", None),
        ("Display (AMOLED)", None),
        ("Display (Super AMOLED)", None),
        ("RAM (LPDDR4)", None),
        ("RAM (LPDDR4X)", None),
        ("RAM (LPDDR5)", None),
        (
            "Memory (eMMC v4.5)",
            Some("embedded MultiMediaCard. Speed: ~140Mb/s"),
        ),
        (
            "Memory (eMMC v5.0)",
            Some("embedded MultiMediaCard. Speed: ~250Mb/s"),
        ),
        (
            "Memory (UFC v2.2)",
            Some("Universal Flash Storage. Speed: ~1200Mb/s"),
        ),
        (
            "Memory (UFC v3.0)",
            Some("Universal Flash Storage. Speed: ~2900Mb/s"),
        ),
        (
            "Memory (UFC v3.1)",
            Some("Universal Flash Storage. Speed: ~2900Mb/s"),
        ),
        ("Display Glass", None),
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

    pub async fn insert(&self, pool: &PgPool) -> Result<PgQueryResult, Error> {
        query(r#"INSERT INTO "ComponentKind" (uuid, name, details) VALUES ($1, $2, $3);"#)
            .bind(self.uuid)
            .bind(self.name.clone())
            .bind(self.details.clone())
            .execute(pool)
            .await
    }
}
