use core::fmt;

use anyhow::{Context, Result};
use sqlx::{postgres::PgQueryResult, query, query_as, Error, PgPool, Postgres};

use crate::{
    tables::Table,
    traits::{MultipleTables, Recreatable},
};

pub mod audit_log;
pub mod update_time;

pub use audit_log::AuditLog;
pub use update_time::UpdateTime;

#[derive(Clone, Copy, Debug)]
pub enum Trigger {
    UpdateTime,
    AuditLog,
}

impl Trigger {
    pub const ALL: [Self; 2] = [Self::UpdateTime, Self::AuditLog];

    pub fn name(&self) -> &str {
        match self {
            Self::UpdateTime => UpdateTime::NAME,
            Self::AuditLog => AuditLog::NAME,
        }
    }

    pub fn create(&self, table: Table) -> String {
        match self {
            Self::UpdateTime => UpdateTime::CREATE.to_string().replace("$1", table.name()),
            Self::AuditLog => AuditLog::CREATE.to_string().replace("$1", table.name()),
        }
    }

    pub fn drop(&self, table: Table) -> String {
        match self {
            Self::UpdateTime => UpdateTime::DROP.to_string().replace("$1", table.name()),
            Self::AuditLog => AuditLog::DROP.to_string().replace("$1", table.name()),
        }
    }

    pub fn tables(&self) -> &[Table] {
        match self {
            Self::UpdateTime => &UpdateTime::TABLES,
            Self::AuditLog => &AuditLog::TABLES,
        }
    }

    pub async fn exists(&self, pool: &PgPool) -> Vec<(Table, Result<bool, Error>)> {
        let mut result = Vec::with_capacity(self.tables().len());

        for table in self.tables() {
            result.push((
                *table,
                match query_as::<Postgres, (bool,)>(
                    r#"SELECT true FROM pg_catalog.pg_trigger trg JOIN pg_catalog.pg_class cls
ON trg.tgrelid = cls."oid" WHERE trg.tgname = $1 AND cls.relname = $2;"#,
                )
                .bind(self.name())
                .bind(table.name())
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
                },
            ))
        }

        result
    }

    /// Create all procedures necessary for application
    pub async fn create_all(
        pool: &PgPool,
        handler: impl Fn((Trigger, Table, Result<PgQueryResult, Error>)) -> Result<PgQueryResult, Error>,
    ) -> Result<()> {
        for trigger in Self::ALL {
            println!("> Creating '{trigger}' trigger");
            for table in trigger.tables() {
                handler((
                    trigger,
                    *table,
                    query(&trigger.create(*table)).execute(pool).await,
                ))
                .with_context(|| {
                    format!("While creating '{trigger}' trigger for '{table}' table")
                })?;
            }
        }

        Ok(())
    }

    /// Drop all application procedures
    pub async fn drop_all(
        pool: &PgPool,
        handler: impl Fn((Trigger, Table, Result<PgQueryResult, Error>)) -> Result<PgQueryResult, Error>,
    ) -> Result<()> {
        for trigger in Self::ALL {
            println!("> Dropping '{trigger}' trigger");
            for table in trigger.tables() {
                handler((
                    trigger,
                    *table,
                    query(&trigger.drop(*table)).execute(pool).await,
                ))
                .with_context(|| format!("While dropping trigger for '{table}' table"))?;
            }
        }

        Ok(())
    }
}

impl fmt::Display for Trigger {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}
