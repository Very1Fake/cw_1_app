use crate::traits::Recreatable;

pub struct ComponentBeautified;

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
