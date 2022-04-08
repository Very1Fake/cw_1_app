use sqlx::types::Uuid;

use crate::types::staff_status::StaffStatus;

#[derive()]
pub struct Staff {
    pub uuid: Uuid,
    /// Foreign key references [`LaborContract`](`super::labor_contract::LaborContract`)
    pub contract: Uuid,
    /// Foreign key references [`Position`](`super::position::Position`)
    pub position: i32,
    pub status: StaffStatus,
}

impl Staff {
    pub const NAME: &'static str = "Staff";

    pub const CREATE: &'static str = r#"CREATE TABLE "Staff" (
    uuid uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    contract uuid NOT NULL REFERENCES "LaborContract" ON DELETE restrict ON UPDATE cascade,
    position int NOT NULL REFERENCES "Position" ON DELETE restrict ON UPDATE cascade,
    status "StaffStatus" NOT NULL DEFAULT 'Suspended'
);"#;

    pub const DROP: &'static str = r#"DROP TABLE "Staff";"#;

    pub const fn new(uuid: Uuid, contract: Uuid, position: i32, status: StaffStatus) -> Self {
        Self {
            uuid,
            contract,
            position,
            status,
        }
    }
}
