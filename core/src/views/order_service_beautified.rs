use crate::traits::Recreatable;

pub struct OrderServiceBeautified;

impl Recreatable for OrderServiceBeautified {
    const NAME: &'static str = "OrderServiceBeautified";
    const CREATE: &'static str = r#"CREATE VIEW "OrderServiceBeautified" AS
SELECT os.order as order,
    s.name as service,
    os.price as price
FROM "OrderService" os,
    "Service" s
WHERE os.service = s.uuid;"#;
    const DROP: &'static str = r#"DROP VIEW "OrderServiceBeautified";"#;
}
