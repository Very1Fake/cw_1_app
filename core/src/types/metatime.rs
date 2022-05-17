use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::Type;

use crate::traits::Recreatable;

#[derive(Type, Serialize, Deserialize, Clone, Debug)]
pub struct MetaTime {
    pub updated: DateTime<Utc>,
    pub created: DateTime<Utc>,
}

impl MetaTime {
    pub fn now() -> DateTime<Utc> {
        Utc::now()
    }
}

impl Recreatable for MetaTime {
    const NAME: &'static str = "metatime";

    const CREATE: &'static str =
        "CREATE TYPE metatime AS (updated timestamptz, created timestamptz);";

    const DROP: &'static str = r#"DROP TYPE metatime;"#;
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
