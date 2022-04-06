#[derive(Debug)]
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
}
