#[derive(Debug)]
pub enum OrderStatus {
    Processing,
    PendingPayment,
    Active,
    Complete,
    Rejected,
}
