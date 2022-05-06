use crate::traits::Recreatable;

pub struct ServicePhoneModelBeautified;

impl Recreatable for ServicePhoneModelBeautified {
    const NAME: &'static str = "ServicePhoneModelBeautified";
    const CREATE: &'static str = r#"CREATE VIEW "ServicePhoneModelBeautified" AS
SELECT s.name as service,
    pm.name as phone_model,
    spm.price as price,
    spm.meta as meta
FROM "ServicePhoneModel" spm,
    "Service" s,
    "PhoneModel" pm
WHERE spm.service = s.uuid
    AND spm.phone_model = pm.uuid;"#;
    const DROP: &'static str = r#"DROP VIEW "ServicePhoneModelBeautified";"#;
}
