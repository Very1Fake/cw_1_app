use core::fmt;

use anyhow::{Context, Result};
use sqlx::{postgres::PgQueryResult, query, query_as, Error, PgPool, Postgres};

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

use crate::traits::Recreatable;

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
            Self::AccountRole => AccountRole::NAME,
            Self::AccountStatus => AccountStatus::NAME,
            Self::Color => Color::NAME,
            Self::ContractStatus => ContractStatus::NAME,
            Self::MetaTime => MetaTime::NAME,
            Self::OrderStatus => OrderStatus::NAME,
            Self::StaffStatus => StaffStatus::NAME,
            Self::SupplyStatus => SupplyStatus::NAME,
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
            Self::AccountRole => AccountRole::CREATE,
            Self::AccountStatus => AccountStatus::CREATE,
            Self::Color => Color::CREATE,
            Self::ContractStatus => ContractStatus::CREATE,
            Self::MetaTime => MetaTime::CREATE,
            Self::OrderStatus => OrderStatus::CREATE,
            Self::StaffStatus => StaffStatus::CREATE,
            Self::SupplyStatus => SupplyStatus::CREATE,
        }
    }

    pub fn drop(&self) -> &str {
        match self {
            Self::AccountRole => AccountRole::DROP,
            Self::AccountStatus => AccountStatus::DROP,
            Self::Color => Color::DROP,
            Self::ContractStatus => ContractStatus::DROP,
            Self::MetaTime => MetaTime::DROP,
            Self::OrderStatus => OrderStatus::DROP,
            Self::StaffStatus => StaffStatus::DROP,
            Self::SupplyStatus => SupplyStatus::DROP,
        }
    }

    /// Create all types necessary for application
    pub async fn create_all(
        pool: &PgPool,
        handler: impl Fn((DbType, Result<PgQueryResult, Error>)) -> Result<PgQueryResult, Error>,
    ) -> Result<()> {
        for db_type in Self::ALL {
            handler((db_type, query(db_type.create()).execute(pool).await))
                .with_context(|| format!("While creating '{db_type}' type"))?;
        }

        Ok(())
    }

    /// Drop all types necessary for application
    pub async fn drop_all(
        pool: &PgPool,
        handler: impl Fn((DbType, Result<PgQueryResult, Error>)) -> Result<PgQueryResult, Error>,
    ) -> Result<()> {
        for db_type in Self::ALL {
            handler((db_type, query(db_type.drop()).execute(pool).await))
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
