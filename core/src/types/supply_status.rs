#[derive(sqlx::Type, Clone, Copy, Debug)]
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
    pub const NAME: &'static str = "SupplyStatus";

    pub const CREATE: &'static str = r#"CREATE TYPE "SupplyStatus" AS ENUM (
    'Review',
    'Negotiation',
    'Signed',
    'Paid',
    'Dispatched',
    'Delivered',
    'Failed',
    'Rejected'
);"#;

    pub const DROP: &'static str = r#"DROP TYPE "SupplyStatus";"#;
}
