use core::fmt;

use anyhow::{Context, Result};
use sqlx::{postgres::PgQueryResult, query, query_as, Error, Executor, PgPool, Postgres};

pub mod account_role;
pub mod account_status;
pub mod color;
pub mod contract_status;
pub mod metatime;
pub mod order_status;
pub mod staff_status;
pub mod supply_status;

pub use account_role::AccountRole;
pub use account_status::AccountStatus;
pub use color::Color;
pub use contract_status::ContractStatus;
pub use metatime::MetaTime;
pub use order_status::OrderStatus;
pub use staff_status::StaffStatus;
pub use supply_status::SupplyStatus;

#[derive(Clone, Copy, Debug)]
pub enum DbType {
    AccountRole,
    AccountStatus,
    Color,
    ContractStatus,
    MetaTime,
    OrderStatus,
    StaffStatus,
    SupplyStatus,
}

impl DbType {
    pub const ALL: [Self; 8] = [
        Self::AccountRole,
        Self::AccountStatus,
        Self::Color,
        Self::ContractStatus,
        Self::MetaTime,
        Self::OrderStatus,
        Self::StaffStatus,
        Self::SupplyStatus,
    ];

    pub fn name(&self) -> &str {
        match self {
            DbType::AccountRole => AccountRole::NAME,
            DbType::AccountStatus => AccountStatus::NAME,
            DbType::Color => Color::NAME,
            DbType::ContractStatus => ContractStatus::NAME,
            DbType::MetaTime => MetaTime::NAME,
            DbType::OrderStatus => OrderStatus::NAME,
            DbType::StaffStatus => StaffStatus::NAME,
            DbType::SupplyStatus => SupplyStatus::NAME,
        }
    }

    pub async fn exists(&self, pool: &PgPool) -> Result<bool, Error> {
        match query_as::<Postgres, (bool,)>(
            "SELECT true FROM pg_catalog.pg_type WHERE typname = $1",
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
            DbType::AccountRole => AccountRole::CREATE,
            DbType::AccountStatus => AccountStatus::CREATE,
            DbType::Color => Color::CREATE,
            DbType::ContractStatus => ContractStatus::CREATE,
            DbType::MetaTime => MetaTime::CREATE,
            DbType::OrderStatus => OrderStatus::CREATE,
            DbType::StaffStatus => StaffStatus::CREATE,
            DbType::SupplyStatus => SupplyStatus::CREATE,
        }
    }

    pub fn drop(&self) -> &str {
        match self {
            DbType::AccountRole => AccountRole::DROP,
            DbType::AccountStatus => AccountStatus::DROP,
            DbType::Color => Color::DROP,
            DbType::ContractStatus => ContractStatus::DROP,
            DbType::MetaTime => MetaTime::DROP,
            DbType::OrderStatus => OrderStatus::DROP,
            DbType::StaffStatus => StaffStatus::DROP,
            DbType::SupplyStatus => SupplyStatus::DROP,
        }
    }

    /// Create all types necessary for application
    pub async fn create_all(
        pool: &PgPool,
        handler: impl Fn((DbType, Result<PgQueryResult, Error>)) -> Result<PgQueryResult, Error>,
    ) -> Result<()> {
        for db_type in DbType::ALL {
            handler((db_type, pool.execute(query(db_type.create())).await))
                .with_context(|| format!("While creating '{db_type}' type"))?;
        }

        Ok(())
    }

    /// Drop all types necessary for application
    pub async fn drop_all(
        pool: &PgPool,
        handler: impl Fn((DbType, Result<PgQueryResult, Error>)) -> Result<PgQueryResult, Error>,
    ) -> Result<()> {
        for db_type in DbType::ALL {
            handler((db_type, pool.execute(query(db_type.drop())).await))
                .with_context(|| format!("While dropping '{db_type}' type"))?;
        }

        Ok(())
    }
}

impl fmt::Display for DbType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}
