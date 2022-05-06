use serde::{Deserialize, Serialize};

use crate::traits::Recreatable;

#[derive(Serialize, Deserialize, sqlx::Type, Clone, Copy, Debug)]
#[sqlx(type_name = "OrderStatus", rename_all = "PascalCase")]
pub enum OrderStatus {
    Processing,
    PendingPayment,
    Active,
    Complete,
    Rejected,
}

impl OrderStatus {
    pub const ALL: [Self; 5] = [
        Self::Processing,
        Self::PendingPayment,
        Self::Active,
        Self::Complete,
        Self::Rejected,
    ];
}

impl Recreatable for OrderStatus {
    const NAME: &'static str = "OrderStatus";

    const CREATE: &'static str = r#"CREATE TYPE "OrderStatus" AS ENUM(
    'Processing',
    'PendingPayment',
    'Active',
    'Complete',
    'Rejected'
);"#;

    const DROP: &'static str = r#"DROP TYPE "OrderStatus";"#;
}
