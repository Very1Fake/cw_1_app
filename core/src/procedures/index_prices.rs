pub struct IndexPrices;

impl IndexPrices {
    pub const NAME: &'static str = "IndexPrices";
    pub const CREATE: &'static str = r#"CREATE PROCEDURE "IndexPrices" (coef real) AS $Body$
UPDATE "Warehouse"
SET price = price * coef;
UPDATE "ServicePhoneModel"
SET price = price * coef;
$Body$ LANGUAGE SQL;"#;
    pub const DROP: &'static str = r#"DROP PROCEDURE "IndexPrices";"#;
}
