#[derive(Debug)]
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
}
