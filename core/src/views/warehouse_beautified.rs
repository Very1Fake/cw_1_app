use crate::traits::Recreatable;

pub struct WarehouseBeautified;

impl Recreatable for WarehouseBeautified {
    const NAME: &'static str = "WarehouseBeautified";
    const CREATE: &'static str = r#"CREATE VIEW "WarehouseBeautified" AS
SELECT w.uuid as uuid,
    c.name as component,
    s.name as supplier,
    w.price as price,
    w.amount as amount,
    w.meta as meta
FROM "Warehouse" w,
    "Supplier" s,
    "Component" c
WHERE w.supplier = s.uuid
    AND w.component = c.uuid;"#;
    const DROP: &'static str = r#"DROP VIEW "WarehouseBeautified";"#;
}
