use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::Type, Clone, Copy, Debug)]
#[sqlx(type_name = "ContractStatus", rename_all = "PascalCase")]
pub enum ContractStatus {
    Review,
    Negotiation,
    Active,
    Expired,
    Void,
    Rejected,
}

impl ContractStatus {
    pub const ALL: [Self; 6] = [
        Self::Review,
        Self::Negotiation,
        Self::Active,
        Self::Expired,
        Self::Void,
        Self::Rejected,
    ];

    pub const NAME: &'static str = "ContractStatus";

    pub const CREATE: &'static str = r#"CREATE TYPE "ContractStatus" AS ENUM(
    'Review',
    'Negotiation',
    'Active',
    'Expired',
    'Void',
    'Rejected'
);"#;

    pub const DROP: &'static str = r#"DROP TYPE "ContractStatus";"#;

    pub fn as_str(&self) -> &str {
        use ContractStatus::*;

        match self {
            Review => "Review",
            Negotiation => "Negotiation",
            Active => "Active",
            Expired => "Expired",
            Void => "Void",
            Rejected => "Rejected",
        }
    }
}
