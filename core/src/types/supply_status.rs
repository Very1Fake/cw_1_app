use serde::{Deserialize, Serialize};

use crate::traits::Recreatable;

#[derive(Serialize, Deserialize, sqlx::Type, Clone, Copy, Debug)]
#[sqlx(type_name = "SupplyStatus", rename_all = "PascalCase")]
pub enum SupplyStatus {
    Review,
    Negotiation,
    Signed,
    Paid,
    Dispatched,
    Delivered,
    Failed,
    Rejected,
}

impl SupplyStatus {
    pub const ALL: [Self; 8] = [
        Self::Review,
        Self::Negotiation,
        Self::Signed,
        Self::Paid,
        Self::Dispatched,
        Self::Delivered,
        Self::Failed,
        Self::Rejected,
    ];
}

impl Recreatable for SupplyStatus {
    const NAME: &'static str = "SupplyStatus";

    const CREATE: &'static str = r#"CREATE TYPE "SupplyStatus" AS ENUM (
    'Review',
    'Negotiation',
    'Signed',
    'Paid',
    'Dispatched',
    'Delivered',
    'Failed',
    'Rejected'
);"#;

    const DROP: &'static str = r#"DROP TYPE "SupplyStatus";"#;
}
