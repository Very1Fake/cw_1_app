pub use argon2;
pub use chrono;
pub use sqlx;
pub use sqlx::types::BigDecimal;
pub use uuid;

pub mod extensions;
pub mod functions;
pub mod generator;
pub mod procedures;
pub mod tables;
pub mod traits;
pub mod types;
pub mod utils;
pub mod views;

// TODO: Dataset generator
// TODO: Batch insert
// TODO: Datetime scattering
// FIX: Replace unwrap() in generators with proper handler
// TODO: Statuses for OrderService and OrderWarehouse
