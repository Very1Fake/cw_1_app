use serde::{Deserialize, Serialize};

use crate::traits::Recreatable;

#[derive(Serialize, Deserialize, sqlx::Type, PartialEq, Clone, Copy, Debug)]
#[sqlx(type_name = "AccountRole", rename_all = "PascalCase")]
pub enum AccountRole {
    Admin,
    Manager,
    #[sqlx(rename = "HR")]
    HR,
    Accountant,
    Serviceman,
    Shopman,
    WarehouseWorker,
}

impl AccountRole {
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

impl Recreatable for AccountRole {
    const NAME: &'static str = "AccountRole";

    const CREATE: &'static str = r#"CREATE TYPE "AccountRole" As ENUM(
    'Admin',
    'Manager',
    'HR',
    'Accountant',
    'Serviceman',
    'Shopman',
    'WarehouseWorker'
);"#;

    const DROP: &'static str = r#"DROP TYPE "AccountRole";"#;
}
