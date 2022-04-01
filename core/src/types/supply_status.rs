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
