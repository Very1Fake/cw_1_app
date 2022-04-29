#[derive(Debug)]
pub struct AuditLog;

impl AuditLog {
    pub const NAME: &'static str = "AuditLog";

    pub const CREATE: &'static str = r#"CREATE TABLE "AuditLog" (
    uuid uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    username text NOT NULL,
    action text NOT NULL,
    tbl text NOT NULL,
    meta metatime NOT NULL DEFAULT (current_timestamp, current_timestamp)
);"#;

    pub const DROP: &'static str = r#"DROP TABLE "AuditLog";"#;
}
