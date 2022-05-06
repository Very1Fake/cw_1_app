use crate::traits::Recreatable;

pub struct AuditLogFunc;

impl Recreatable for AuditLogFunc {
    const NAME: &'static str = "audit_log_func";

    const CREATE: &'static str = r#"CREATE OR REPLACE FUNCTION audit_log_func() RETURNS trigger AS
$$
DECLARE
    tbl text := TG_TABLE_NAME;
    op text := TG_OP;
BEGIN
    INSERT INTO "AuditLog" (username, action, tbl) VALUES (current_user, op, tbl);

    RETURN NEW;
END;
$$ LANGUAGE PLPGSQL;"#;

    const DROP: &'static str = r#"DROP FUNCTION "audit_log_func";"#;
}
