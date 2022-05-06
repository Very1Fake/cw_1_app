use crate::traits::Recreatable;

pub struct StaffBeautified;

impl Recreatable for StaffBeautified {
    const NAME: &'static str = "StaffBeautified";
    const CREATE: &'static str = r#"CREATE VIEW "StaffBeautified" AS
SELECT s.uuid,
    s.contract,
    p.name,
    s.status
FROM "Staff" s,
    "Position" p
WHERE s.position = p.uuid;"#;
    const DROP: &'static str = r#"DROP VIEW "StaffBeautified";"#;
}
