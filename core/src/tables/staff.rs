use uuid::Uuid;

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
    pub const fn new(uuid: Uuid, contract: Uuid, position: i32, status: StaffStatus) -> Self {
        Self {
            uuid,
            contract,
            position,
            status,
        }
    }
}
