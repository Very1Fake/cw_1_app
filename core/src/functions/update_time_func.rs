use crate::traits::Recreatable;

pub struct UpdateTimeFunc;

impl Recreatable for UpdateTimeFunc {
    const NAME: &'static str = "update_time_func";

    const CREATE: &'static str = r#"CREATE OR REPLACE FUNCTION update_time_func() RETURNS trigger AS
$$
DECLARE
    tbl text := quote_ident(TG_TABLE_NAME);
BEGIN
    EXECUTE 'UPDATE ' || tbl || '
    SET meta.updated = now()
    WHERE uuid = ''' || OLD.uuid || ''';';

    RETURN NEW;
END;
$$ LANGUAGE PLPGSQL;"#;

    const DROP: &'static str = r#"DROP FUNCTION "update_time_func";"#;
}
