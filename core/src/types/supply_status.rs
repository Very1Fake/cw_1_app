#[derive(Debug)]
pub enum SupplyStatus {
    Review,
    Negotiation,
    Signed,
    Paid,
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
    'Delivered',
    'Failed',
    'Rejected'
);"#;

    pub const DROP: &'static str = r#"DROP TYPE "SupplyStatus";"#;
}
