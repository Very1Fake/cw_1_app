#[derive(sqlx::Type, Clone, Copy, Debug)]
#[sqlx(type_name = "StaffStatus", rename_all = "PascalCase")]
pub enum StaffStatus {
    Working,
    Fired,
    OnVacation,
    Suspended,
}

impl StaffStatus {
    pub const NAME: &'static str = "StaffStatus";

    pub const CREATE: &'static str = r#"CREATE TYPE "StaffStatus" AS ENUM(
'Working', 'Fired', 'OnVacation', 'Suspended');"#;

    pub const DROP: &'static str = r#"DROP TYPE "StaffStatus";"#;

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
