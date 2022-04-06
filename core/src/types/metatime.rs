use std::time::Instant;

#[derive(Debug)]
pub struct MetaTime {
    // FIX: To normal datetime
    pub updated: Instant,
    pub created: Instant,
}

impl MetaTime {
    pub const NAME: &'static str = "metatime";

    pub const CREATE: &'static str =
        "CREATE TYPE metatime AS (updated timestamp, created timestamp);";

    pub const DROP: &'static str = r#"DROP TYPE metatime;"#;
}
