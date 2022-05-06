use core::fmt;

use anyhow::{Context, Result};
use sqlx::{postgres::PgQueryResult, query, query_as, Error, PgPool, Postgres};

use crate::traits::Recreatable;

pub mod index_prices;

pub use index_prices::IndexPrices;

#[derive(Clone, Copy, Debug)]
pub enum Procedure {
    IndexPrices,
}

impl Procedure {
    pub const ALL: [Self; 1] = [Self::IndexPrices];

    pub fn name(&self) -> &str {
        match self {
            Self::IndexPrices => IndexPrices::NAME,
        }
    }

    pub fn create(&self) -> &str {
        match self {
            Self::IndexPrices => IndexPrices::CREATE,
        }
    }

    pub fn drop(&self) -> &str {
        match self {
            Self::IndexPrices => IndexPrices::DROP,
        }
    }

    pub async fn exists(&self, pool: &PgPool) -> Result<bool, Error> {
        match query_as::<Postgres, (bool,)>(
            r#"SELECT true
FROM pg_catalog.pg_proc
    JOIN pg_namespace ON pg_catalog.pg_proc.pronamespace = pg_namespace.oid
WHERE proname = $1
    AND nspname = 'public';"#,
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

    /// Create all procedures necessary for application
    pub async fn create_all(
        pool: &PgPool,
        handler: impl Fn((Procedure, Result<PgQueryResult, Error>)) -> Result<PgQueryResult, Error>,
    ) -> Result<()> {
        for procedure in Self::ALL {
            handler((procedure, query(procedure.create()).execute(pool).await))
                .with_context(|| format!("While creating '{procedure}' procedure"))?;
        }

        Ok(())
    }

    /// Drop all application procedures
    pub async fn drop_all(
        pool: &PgPool,
        handler: impl Fn((Procedure, Result<PgQueryResult, Error>)) -> Result<PgQueryResult, Error>,
    ) -> Result<()> {
        for procedure in Self::ALL {
            handler((procedure, query(procedure.drop()).execute(pool).await))
                .with_context(|| format!("While dropping '{procedure}' procedure"))?;
        }

        Ok(())
    }
}

impl fmt::Display for Procedure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}
