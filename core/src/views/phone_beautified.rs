use crate::traits::Recreatable;

pub struct PhoneBeautified;

impl Recreatable for PhoneBeautified {
    const NAME: &'static str = "PhoneBeautified";
    const CREATE: &'static str = r#"CREATE VIEW "PhoneBeautified" AS
SELECT ph.uuid as uuid,
    CONCAT_WS(' ', p.first_name, p.middle_name, p.last_name) as name,
    ph.imei as imei,
    ph.wifi as wifi,
    ph.bluetooth as bluetooth,
    pm.name as model,
    ph.color as color,
    ph.meta as meta
FROM "Phone" ph,
    "Person" p,
    "PhoneModel" pm
WHERE ph.person = p.uuid
    AND ph.model = pm.uuid;"#;
    const DROP: &'static str = r#"DROP VIEW "PhoneBeautified";"#;
}
