use std::time::Instant;

#[derive(Debug)]
pub struct MetaTime {
    // FIX: To normal datetime
    pub updated: Instant,
    pub created: Instant,
}
