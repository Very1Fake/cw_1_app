#[derive(Debug)]
pub enum ContractStatus {
    Review,
    Negotiation,
    Active,
    Expired,
    Void,
}

impl ContractStatus {
    pub const NAME: &'static str = "ContractStatus";

    pub const CREATE: &'static str = r#"CREATE TYPE "ContractStatus" AS ENUM(
    'Review',
    'Negotiation',
    'Active',
    'Expired',
    'Void'
);"#;

    pub const DROP: &'static str = r#"DROP TYPE "ContractStatus";"#;
}
