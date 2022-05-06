use core::fmt;

use anyhow::{Context, Result};
use sqlx::{postgres::PgQueryResult, query, query_as, Error, PgPool, Postgres};

use crate::traits::Recreatable;

pub mod component_beautified;
pub mod order_service_beautified;
pub mod phone_beautified;
pub mod service_phone_model_beautified;
pub mod staff_beautified;
pub mod warehouse_beautified;

pub use component_beautified::ComponentBeautified;
pub use order_service_beautified::OrderServiceBeautified;
pub use phone_beautified::PhoneBeautified;
pub use service_phone_model_beautified::ServicePhoneModelBeautified;
pub use staff_beautified::StaffBeautified;
pub use warehouse_beautified::WarehouseBeautified;

#[derive(Clone, Copy, Debug)]
pub enum View {
    StaffBeautified,
    ComponentBeautified,
    PhoneBeautified,
    WarehouseBeautified,
    ServicePhoneModelBeautified,
    OrderServiceBeautified,
}

impl View {
    pub const ALL: [Self; 6] = [
        Self::StaffBeautified,
        Self::ComponentBeautified,
        Self::PhoneBeautified,
        Self::WarehouseBeautified,
        Self::ServicePhoneModelBeautified,
        Self::OrderServiceBeautified,
    ];

    pub fn name(&self) -> &str {
        match self {
            Self::StaffBeautified => StaffBeautified::NAME,
            Self::ComponentBeautified => ComponentBeautified::NAME,
            Self::PhoneBeautified => PhoneBeautified::NAME,
            Self::WarehouseBeautified => WarehouseBeautified::NAME,
            Self::ServicePhoneModelBeautified => ServicePhoneModelBeautified::NAME,
            Self::OrderServiceBeautified => OrderServiceBeautified::NAME,
        }
    }

    pub fn create(&self) -> &str {
        match self {
            Self::StaffBeautified => StaffBeautified::CREATE,
            Self::ComponentBeautified => ComponentBeautified::CREATE,
            Self::PhoneBeautified => PhoneBeautified::CREATE,
            Self::WarehouseBeautified => WarehouseBeautified::CREATE,
            Self::ServicePhoneModelBeautified => ServicePhoneModelBeautified::CREATE,
            Self::OrderServiceBeautified => OrderServiceBeautified::CREATE,
        }
    }

    pub fn drop(&self) -> &str {
        match self {
            Self::StaffBeautified => StaffBeautified::DROP,
            Self::ComponentBeautified => ComponentBeautified::DROP,
            Self::PhoneBeautified => PhoneBeautified::DROP,
            Self::WarehouseBeautified => WarehouseBeautified::DROP,
            Self::ServicePhoneModelBeautified => ServicePhoneModelBeautified::DROP,
            Self::OrderServiceBeautified => OrderServiceBeautified::DROP,
        }
    }

    pub async fn exists(&self, pool: &PgPool) -> Result<bool, Error> {
        match query_as::<Postgres, (bool,)>(
            r#"SELECT true FROM information_schema.tables 
WHERE table_schema = 'public' and table_type = 'VIEW' and table_name = $1;"#,
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

    /// Create all views necessary for application
    pub async fn create_all(
        pool: &PgPool,
        handler: impl Fn((View, Result<PgQueryResult, Error>)) -> Result<PgQueryResult, Error>,
    ) -> Result<()> {
        for procedure in Self::ALL {
            handler((procedure, query(procedure.create()).execute(pool).await))
                .with_context(|| format!("While creating '{procedure}' procedure"))?;
        }

        Ok(())
    }

    /// Drop all application views
    pub async fn drop_all(
        pool: &PgPool,
        handler: impl Fn((View, Result<PgQueryResult, Error>)) -> Result<PgQueryResult, Error>,
    ) -> Result<()> {
        for procedure in Self::ALL {
            handler((procedure, query(procedure.drop()).execute(pool).await))
                .with_context(|| format!("While dropping '{procedure}' procedure"))?;
        }

        Ok(())
    }
}

impl fmt::Display for View {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}
