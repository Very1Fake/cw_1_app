#[derive(sqlx::Type, Clone, Copy, Debug)]
#[sqlx(type_name = "AccountStatus", rename_all = "PascalCase")]
pub enum AccountStatus {
    Active,
    Expired,
    Inactive,
}

impl AccountStatus {
    pub const NAME: &'static str = "AccountStatus";

    pub const CREATE: &'static str = r#"CREATE TYPE "AccountStatus" AS ENUM(
    'Active',
    'Expired',
    'Inactive'
);"#;

    pub const DROP: &'static str = r#"DROP TYPE "AccountStatus";"#;

    pub fn as_str(&self) -> &str {
        use AccountStatus::*;

        match self {
            Active => "Active",
            Expired => "Expired",
            Inactive => "Inactive",
        }
    }
}
