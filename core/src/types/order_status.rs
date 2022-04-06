#[derive(Debug)]
pub enum OrderStatus {
    Processing,
    PendingPayment,
    Active,
    Complete,
    Rejected,
}

impl OrderStatus {
    pub const NAME: &'static str = "OrderStatus";

    pub const CREATE: &'static str = r#"CREATE TYPE "OrderStatus" AS ENUM(
    'Processing',
    'PendingPayment',
    'Active',
    'Complete',
    'Rejected'
);"#;

    pub const DROP: &'static str = r#"DROP TYPE "OrderStatus";"#;
}
