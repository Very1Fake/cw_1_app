use core::fmt;

use anyhow::{Context, Result};
use sqlx::{postgres::PgQueryResult, query, query_as, Error, PgPool, Postgres};

use crate::traits::Recreatable;

pub mod audit_log_func;
pub mod revenue_for_period;
pub mod update_time_func;

pub use audit_log_func::AuditLogFunc;
pub use revenue_for_period::RevenueForPeriod;
pub use update_time_func::UpdateTimeFunc;

#[derive(Clone, Copy, Debug)]
pub enum Function {
    UpdateTimeFunc,
    AuditLogFunc,
    RevenueForPeriod,
}

impl Function {
    pub const ALL: [Self; 3] = [
        Self::UpdateTimeFunc,
        Self::AuditLogFunc,
        Self::RevenueForPeriod,
    ];

    pub fn name(&self) -> &str {
        match self {
            Self::UpdateTimeFunc => UpdateTimeFunc::NAME,
            Self::AuditLogFunc => AuditLogFunc::NAME,
            Self::RevenueForPeriod => RevenueForPeriod::NAME,
        }
    }

    pub fn create(&self) -> &str {
        match self {
            Self::UpdateTimeFunc => UpdateTimeFunc::CREATE,
            Self::AuditLogFunc => AuditLogFunc::CREATE,
            Self::RevenueForPeriod => RevenueForPeriod::CREATE,
        }
    }

    pub fn drop(&self) -> &str {
        match self {
            Self::UpdateTimeFunc => UpdateTimeFunc::DROP,
            Self::AuditLogFunc => AuditLogFunc::DROP,
            Self::RevenueForPeriod => RevenueForPeriod::DROP,
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
        handler: impl Fn((Function, Result<PgQueryResult, Error>)) -> Result<PgQueryResult, Error>,
    ) -> Result<()> {
        for function in Self::ALL {
            handler((function, query(function.create()).execute(pool).await))
                .with_context(|| format!("While creating '{function}' function"))?;
        }

        Ok(())
    }

    /// Drop all application procedures
    pub async fn drop_all(
        pool: &PgPool,
        handler: impl Fn((Function, Result<PgQueryResult, Error>)) -> Result<PgQueryResult, Error>,
    ) -> Result<()> {
        for function in Self::ALL {
            handler((function, query(function.drop()).execute(pool).await))
                .with_context(|| format!("While dropping '{function}' function"))?;
        }

        Ok(())
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}
