use crate::traits::Recreatable;

pub struct RevenueForPeriod;

impl Recreatable for RevenueForPeriod {
    const NAME: &'static str = "revenue_for_period";

    const CREATE: &'static str = r#"CREATE FUNCTION "revenue_for_period" (from_date timestamptz, to_date timestamptz) RETURNS TABLE(services money, components money, summary money) AS $$
DECLARE 
    from_services money;
    from_components money;
BEGIN
    SELECT sum(price) INTO from_services
    FROM "OrderService" os
        join "Order" o ON os."order" = o.uuid
    WHERE o.status IN ('Active', 'Complete')
        AND (o.meta).created BETWEEN from_date and to_date;

    SELECT sum(price) INTO from_components
    FROM "OrderWarehouse" ow
        join "Order" o ON ow."order" = o.uuid
    WHERE o.status IN ('Active', 'Complete')
        AND (o.meta).created BETWEEN from_date and to_date;

    RETURN QUERY
        SELECT from_services, from_components, (from_services + from_components) as summary;
END;
$$ LANGUAGE 'plpgsql';"#;

    const DROP: &'static str = r#"DROP FUNCTION "revenue_for_period";"#;
}
