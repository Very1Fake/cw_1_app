#[derive(Debug)]
pub enum AccountRole {
    Admin,
    Manager,
    HR,
    Accountant,
    Serviceman,
    Shopman,
    WarehouseWorker,
}

impl AccountRole {
    pub const NAME: &'static str = "AccountRole";

    pub const CREATE: &'static str = r#"CREATE TYPE "AccountRole" As ENUM(
    'Admin',
    'Manager',
    'HR',
    'Accountant',
    'Serviceman',
    'Shopman',
    'WarehouseWorker'
);"#;

    pub const DROP: &'static str = r#"DROP TYPE "AccountRole";"#;
}
