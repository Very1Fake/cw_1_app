use sqlx::{postgres::PgQueryResult, query, Error, PgPool};
use uuid::Uuid;

use crate::types::staff_status::StaffStatus;

#[derive()]
pub struct Staff {
    pub uuid: Uuid,
    /// Foreign key references [`LaborContract`](`super::labor_contract::LaborContract`)
    pub contract: Uuid,
    /// Foreign key references [`Position`](`super::position::Position`)
    pub position: Uuid,
    pub status: StaffStatus,
}

impl Staff {
    pub const NAME: &'static str = "Staff";

    pub const CREATE: &'static str = r#"CREATE TABLE "Staff" (
    uuid uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    contract uuid NOT NULL REFERENCES "LaborContract" ON DELETE restrict ON UPDATE cascade,
    position uuid NOT NULL REFERENCES "Position" ON DELETE restrict ON UPDATE cascade,
    status "StaffStatus" NOT NULL DEFAULT 'Suspended'
);"#;

    pub const DROP: &'static str = r#"DROP TABLE "Staff";"#;

    pub const fn new(uuid: Uuid, contract: Uuid, position: Uuid, status: StaffStatus) -> Self {
        Self {
            uuid,
            contract,
            position,
            status,
        }
    }

    pub fn new_auto(contract: Uuid, position: Uuid, status: StaffStatus) -> Self {
        Self::new(Uuid::new_v4(), contract, position, status)
    }

    pub async fn insert(&self, pool: &PgPool) -> Result<PgQueryResult, Error> {
        query(
            r#"INSERT INTO "Staff" (uuid, contract, position, status) 
    VALUES ($1, $2, $3, $4);"#,
        )
        .bind(self.uuid)
        .bind(self.contract)
        .bind(self.position)
        .bind(self.status)
        .execute(pool)
        .await
    }
}
