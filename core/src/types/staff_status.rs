use serde::{Deserialize, Serialize};

use crate::traits::Recreatable;

#[derive(Serialize, Deserialize, sqlx::Type, Clone, Copy, Debug)]
#[sqlx(type_name = "StaffStatus", rename_all = "PascalCase")]
pub enum StaffStatus {
    Working,
    Fired,
    OnVacation,
    Suspended,
}

impl StaffStatus {
    pub fn as_str(&self) -> &str {
        use StaffStatus::*;

        match self {
            Working => "Working",
            Fired => "Fired",
            OnVacation => "OnVacation",
            Suspended => "Suspended",
        }
    }
}

impl Recreatable for StaffStatus {
    const NAME: &'static str = "StaffStatus";

    const CREATE: &'static str = r#"CREATE TYPE "StaffStatus" AS ENUM(
'Working', 'Fired', 'OnVacation', 'Suspended');"#;

    const DROP: &'static str = r#"DROP TYPE "StaffStatus";"#;
}
