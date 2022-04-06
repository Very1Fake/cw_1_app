use core::fmt;

use anyhow::{Context, Result};
use sqlx::{postgres::PgQueryResult, query, Error, Executor, PgPool};

pub use uuid_ossp::UuidOssp;

pub const CREATE_ALL: [(&str, &str); 1] = [(UuidOssp::CREATE, UuidOssp::NAME)];
pub const DROP_ALL: [(&str, &str); 1] = [(UuidOssp::DROP, UuidOssp::NAME)];

#[derive(Clone, Copy, Debug)]
pub enum Extension {
    UuidOssp,
}

impl Extension {
    pub const ALL: [Self; 1] = [Self::UuidOssp];

    pub fn name(&self) -> &str {
        match self {
            Extension::UuidOssp => UuidOssp::NAME,
        }
    }

    pub fn create(&self) -> &str {
        match self {
            Extension::UuidOssp => UuidOssp::CREATE,
        }
    }

    pub fn drop(&self) -> &str {
        match self {
            Extension::UuidOssp => UuidOssp::DROP,
        }
    }
}

impl fmt::Display for Extension {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

// -------------------------------------------------------------------------------------------------

/// Loads all postgresql extensions necessary for application
pub async fn ex_create_all(
    pool: &PgPool,
    handler: impl Fn((Extension, Result<PgQueryResult, Error>)) -> Result<PgQueryResult, Error>,
) -> Result<()> {
    for extension in Extension::ALL {
        handler((extension, pool.execute(query(extension.create())).await))
            .with_context(|| format!("While loading '{extension}' extension"))?;
    }

    Ok(())
}

// TODO: Check for ownership
/// Unloads all postgresql extensions
pub async fn ex_drop_all(
    pool: &PgPool,
    handler: impl Fn((Extension, Result<PgQueryResult, Error>)) -> Result<PgQueryResult, Error>,
) -> Result<()> {
    for extension in Extension::ALL {
        handler((extension, pool.execute(query(extension.drop())).await))
            .with_context(|| format!("While unloading '{extension}' extension"))?;
    }

    Ok(())
}

// -------------------------------------------------------------------------------------------------

mod uuid_ossp {

    pub struct UuidOssp;

    impl UuidOssp {
        pub const NAME: &'static str = "uuid-ossp";
        pub const CREATE: &'static str = r#"CREATE EXTENSION IF NOT EXISTS "uuid-ossp";"#;
        pub const DROP: &'static str = r#"DROP EXTENSION IF EXISTS "uuid-ossp";"#;
    }
}
