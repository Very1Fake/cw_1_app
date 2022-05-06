use crate::{
    tables::Table,
    traits::{MultipleTables, Recreatable},
};

pub struct AuditLog;

impl Recreatable for AuditLog {
    const NAME: &'static str = "audit_log";
    const CREATE: &'static str = r#"CREATE TRIGGER audit_log AFTER
UPDATE OR INSERT OR DELETE ON "$1" FOR EACH ROW
WHEN (pg_trigger_depth() = 0) EXECUTE FUNCTION audit_log_func();"#;
    const DROP: &'static str = r#"DROP TRIGGER audit_log ON "$1";"#;
}

impl MultipleTables<21> for AuditLog {
    const TABLES: [Table; 21] = Table::ALL;
}
