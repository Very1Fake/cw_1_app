use serde::{Deserialize, Serialize};

use crate::traits::Recreatable;

#[derive(Serialize, Deserialize, sqlx::Type, Clone, Copy, Debug)]
#[sqlx(type_name = "AccountStatus", rename_all = "PascalCase")]
pub enum AccountStatus {
    Active,
    Expired,
    Inactive,
}

impl AccountStatus {
    pub const ALL: [Self; 3] = [Self::Active, Self::Expired, Self::Inactive];

    pub fn as_str(&self) -> &str {
        use AccountStatus::*;

        match self {
            Active => "Active",
            Expired => "Expired",
            Inactive => "Inactive",
        }
    }
}

impl Recreatable for AccountStatus {
    const NAME: &'static str = "AccountStatus";

    const CREATE: &'static str = r#"CREATE TYPE "AccountStatus" AS ENUM(
    'Active',
    'Expired',
    'Inactive'
);"#;

    const DROP: &'static str = r#"DROP TYPE "AccountStatus";"#;
}
