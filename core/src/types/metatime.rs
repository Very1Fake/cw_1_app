use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct MetaTime {
    pub updated: DateTime<Utc>,
    pub created: DateTime<Utc>,
}

impl MetaTime {
    pub const NAME: &'static str = "metatime";

    pub const CREATE: &'static str =
        "CREATE TYPE metatime AS (updated timestamptz, created timestamptz);";

    pub const DROP: &'static str = r#"DROP TYPE metatime;"#;

    pub fn now() -> DateTime<Utc> {
        Utc::now()
    }
}

impl Default for MetaTime {
    fn default() -> Self {
        let now = Self::now();
        Self {
            updated: now,
            created: now,
        }
    }
}
