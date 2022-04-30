use core::fmt;

use anyhow::{Context, Result};
use sqlx::{postgres::PgDatabaseError, query, query_as, Error, PgPool, Postgres};

pub use uuid_ossp::UuidOssp;

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

    pub async fn exists(&self, pool: &PgPool) -> Result<bool, Error> {
        match query_as::<Postgres, (bool,)>(
            "SELECT true FROM pg_catalog.pg_extension WHERE extname = $1",
        )
        .bind(self.name())
        .fetch_one(pool)
        .await
        {
            Ok(_) => Ok(true),
            Err(err) => {
                if let Error::RowNotFound = err {
                    Ok(false)
                } else {
                    Err(err)
                }
            }
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

    /// Loads all postgresql extensions necessary for application
    pub async fn create_all(pool: &PgPool, printer: impl Fn((Extension, bool))) -> Result<()> {
        for extension in Self::ALL {
            match query(extension.create()).execute(pool).await {
                Ok(_) => printer((extension, true)),
                Err(err) => {
                    if let Error::Database(err) = &err {
                        if err.downcast_ref::<PgDatabaseError>().code() == "42710" {
                            printer((extension, true)); // TODO: Explicit alert
                            continue;
                        }
                    }
                    printer((extension, false));
                    return Err(err)
                        .with_context(|| format!("While loading '{extension}' extension"));
                }
            }
        }

        Ok(())
    }

    /// Unloads all postgresql extensions
    pub async fn drop_all(pool: &PgPool, printer: impl Fn((Extension, bool))) -> Result<()> {
        for extension in Self::ALL {
            match query(extension.drop()).execute(pool).await {
                Ok(_) => printer((extension, true)),
                Err(err) => {
                    if let Error::Database(err) = &err {
                        if err.downcast_ref::<PgDatabaseError>().code() == "42501" {
                            printer((extension, true)); // TODO: Explicit alert
                            continue;
                        }
                    }

                    printer((extension, false));
                    return Err(err)
                        .with_context(|| format!("While unloading '{extension}' extension"));
                }
            }
        }

        Ok(())
    }
}

impl fmt::Display for Extension {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

// -------------------------------------------------------------------------------------------------

mod uuid_ossp {

    pub struct UuidOssp;

    impl UuidOssp {
        pub const NAME: &'static str = "uuid-ossp";
        pub const CREATE: &'static str = r#"CREATE EXTENSION "uuid-ossp";"#;
        pub const DROP: &'static str = r#"DROP EXTENSION "uuid-ossp";"#;
    }
}
