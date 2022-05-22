use sqlx::{query_as, FromRow};
use uuid::Uuid;

use crate::{traits::Recreatable, PgQueryAs};

#[derive(FromRow, Clone, Debug)]
pub struct ComponentBeautified {
    pub uuid: Uuid,
    pub name: String,
    pub kind: String,
    pub model: String,
    pub manufacturer: String,
}

impl Recreatable for ComponentBeautified {
    const NAME: &'static str = "ComponentBeautified";
    const CREATE: &'static str = r#"CREATE VIEW "ComponentBeautified" AS
SELECT c.uuid as uuid,
    c.name as name,
    k.name as kind,
    pm.name as model,
    m.name as manufacturer
FROM "Component" c,
    "ComponentKind" k,
    "PhoneModel" pm,
    "Manufacturer" m
WHERE c.kind = k.uuid
    AND c.manufacturer = m.uuid
    AND c.phone_model = pm.uuid;"#;
    const DROP: &'static str = r#"DROP VIEW "ComponentBeautified";"#;
}

impl ComponentBeautified {
    pub fn get_all() -> PgQueryAs<Self> {
        query_as(r#"SELECT * FROM "ComponentBeautified""#)
    }
}
