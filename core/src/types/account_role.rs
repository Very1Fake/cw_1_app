#[derive(sqlx::Type, Clone, Copy, Debug)]
#[sqlx(type_name = "AccountRole", rename_all = "PascalCase")]
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

    pub fn as_str(&self) -> &str {
        use AccountRole::*;

        match self {
            Admin => "Admin",
            Manager => "Manager",
            HR => "HR",
            Accountant => "Accountant",
            Serviceman => "Serviceman",
            Shopman => "Shopman",
            WarehouseWorker => "WarehouseWorker",
        }
    }
}
