use crate::traits::Recreatable;

pub struct IndexPrices;

impl Recreatable for IndexPrices {
    const NAME: &'static str = "IndexPrices";
    const CREATE: &'static str = r#"CREATE PROCEDURE "IndexPrices" (coef real) AS $$
UPDATE "Warehouse"
SET price = price * coef;
UPDATE "ServicePhoneModel"
SET price = price * coef;
$$ LANGUAGE SQL;"#;
    const DROP: &'static str = r#"DROP PROCEDURE "IndexPrices";"#;
}
