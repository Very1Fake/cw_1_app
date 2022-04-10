use core::fmt;

use anyhow::{Context, Result};
use sqlx::{postgres::PgQueryResult, query, query_as, Error, Executor, PgPool, Postgres};

pub mod account;
pub mod component;
pub mod component_kind;
pub mod labor_contract;
pub mod manufacturer;
pub mod order;
pub mod order_component;
pub mod order_service;
pub mod person;
pub mod phone;
pub mod phone_model;
pub mod position;
pub mod service;
pub mod service_phone_model;
pub mod staff;
pub mod supplier;
pub mod supply;
pub mod supply_contract;
pub mod warehouse;
pub mod warehouse_supply;

pub use account::Account;
pub use component::Component;
pub use component_kind::ComponentKind;
pub use labor_contract::LaborContract;
pub use manufacturer::Manufacturer;
pub use order::Order;
pub use order_component::OrderWarehouse;
pub use order_service::OrderService;
pub use person::Person;
pub use phone::Phone;
pub use phone_model::PhoneModel;
pub use position::Position;
pub use service::Service;
pub use service_phone_model::ServicePhoneModel;
pub use staff::Staff;
pub use supplier::Supplier;
pub use supply::Supply;
pub use supply_contract::SupplyContract;
pub use warehouse::Warehouse;
pub use warehouse_supply::WarehouseSupply;

// -------------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Debug)]
pub enum Table {
    Person,
    Supplier,
    Manufacturer,
    Position,
    Service,
    ComponentKind,
    LaborContract,
    Staff,
    PhoneModel,
    Phone,
    SupplyContract,
    Supply,
    Account,
    Component,
    Warehouse,
    Order,
    ServicePhoneModel,
    WarehouseSupply,
    OrderService,
    OrderWarehouse,
}

impl Table {
    pub const LOW: [Self; 6] = [
        Self::Person,
        Self::Supplier,
        Self::Manufacturer,
        Self::Position,
        Self::Service,
        Self::ComponentKind,
    ];

    pub const ALL: [Self; 20] = [
        // Low-level tables
        Self::Person,
        Self::Supplier,
        Self::Manufacturer,
        Self::Position,
        Self::Service,
        Self::ComponentKind,
        // Mid-level tables
        Self::LaborContract,
        Self::Staff,
        Self::PhoneModel,
        Self::Phone,
        Self::SupplyContract,
        Self::Supply,
        // High-level tables
        Self::Account,
        Self::Component,
        Self::Warehouse,
        Self::Order,
        // Relations tables
        Self::ServicePhoneModel,
        Self::WarehouseSupply,
        Self::OrderService,
        Self::OrderWarehouse,
    ];

    pub fn name(&self) -> &str {
        match self {
            Table::Person => Person::NAME,
            Table::Supplier => Supplier::NAME,
            Table::Manufacturer => Manufacturer::NAME,
            Table::Position => Position::NAME,
            Table::Service => Service::NAME,
            Table::ComponentKind => ComponentKind::NAME,
            Table::LaborContract => LaborContract::NAME,
            Table::Staff => Staff::NAME,
            Table::PhoneModel => PhoneModel::NAME,
            Table::Phone => Phone::NAME,
            Table::SupplyContract => SupplyContract::NAME,
            Table::Supply => Supply::NAME,
            Table::Account => Account::NAME,
            Table::Component => Component::NAME,
            Table::Warehouse => Warehouse::NAME,
            Table::Order => Order::NAME,
            Table::ServicePhoneModel => ServicePhoneModel::NAME,
            Table::WarehouseSupply => WarehouseSupply::NAME,
            Table::OrderService => OrderService::NAME,
            Table::OrderWarehouse => OrderWarehouse::NAME,
        }
    }

    pub fn create(&self) -> &str {
        match self {
            Table::Person => Person::CREATE,
            Table::Supplier => Supplier::CREATE,
            Table::Manufacturer => Manufacturer::CREATE,
            Table::Position => Position::CREATE,
            Table::Service => Service::CREATE,
            Table::ComponentKind => ComponentKind::CREATE,
            Table::LaborContract => LaborContract::CREATE,
            Table::Staff => Staff::CREATE,
            Table::PhoneModel => PhoneModel::CREATE,
            Table::Phone => Phone::CREATE,
            Table::SupplyContract => SupplyContract::CREATE,
            Table::Supply => Supply::CREATE,
            Table::Account => Account::CREATE,
            Table::Component => Component::CREATE,
            Table::Warehouse => Warehouse::CREATE,
            Table::Order => Order::CREATE,
            Table::ServicePhoneModel => ServicePhoneModel::CREATE,
            Table::WarehouseSupply => WarehouseSupply::CREATE,
            Table::OrderService => OrderService::CREATE,
            Table::OrderWarehouse => OrderWarehouse::CREATE,
        }
    }

    pub fn drop(&self) -> &str {
        match self {
            Table::Person => Person::DROP,
            Table::Supplier => Supplier::DROP,
            Table::Manufacturer => Manufacturer::DROP,
            Table::Position => Position::DROP,
            Table::Service => Service::DROP,
            Table::ComponentKind => ComponentKind::DROP,
            Table::LaborContract => LaborContract::DROP,
            Table::Staff => Staff::DROP,
            Table::PhoneModel => PhoneModel::DROP,
            Table::Phone => Phone::DROP,
            Table::SupplyContract => SupplyContract::DROP,
            Table::Supply => Supply::DROP,
            Table::Account => Account::DROP,
            Table::Component => Component::DROP,
            Table::Warehouse => Warehouse::DROP,
            Table::Order => Order::DROP,
            Table::ServicePhoneModel => ServicePhoneModel::DROP,
            Table::WarehouseSupply => WarehouseSupply::DROP,
            Table::OrderService => OrderService::DROP,
            Table::OrderWarehouse => OrderWarehouse::DROP,
        }
    }

    pub fn truncate_query(&self) -> String {
        format!(r#"TRUNCATE "{}" RESTART IDENTITY cascade;"#, self.name())
    }

    pub async fn exists(&self, pool: &PgPool) -> Result<bool, Error> {
        match query_as::<Postgres, (bool,)>(
            "SELECT true FROM information_schema.tables WHERE table_name = $1",
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

    /// Creates all application tables
    pub async fn create_all(
        pool: &PgPool,
        handler: impl Fn((Table, Result<PgQueryResult, Error>)) -> Result<PgQueryResult, Error>,
    ) -> Result<()> {
        for table in Self::ALL {
            handler((table, pool.execute(query(table.create())).await))
                .with_context(|| format!("While creating '{table}' table"))?;
        }

        Ok(())
    }

    /// Drops all application tables
    pub async fn drop_all(
        pool: &PgPool,
        handler: impl Fn((Table, Result<PgQueryResult, Error>)) -> Result<PgQueryResult, Error>,
    ) -> Result<()> {
        for table in Self::ALL.into_iter().rev() {
            handler((table, query(table.drop()).execute(pool).await))
                .with_context(|| format!("While dropping '{table}' table"))?;
        }

        Ok(())
    }

    /// Truncates all tables!!!
    pub async fn truncate(pool: &PgPool, handler: impl Fn((Table, bool))) -> Result<()> {
        for table in Self::LOW.into_iter() {
            match query(table.truncate_query().as_str()).execute(pool).await {
                Ok(_) => handler((table, true)),
                Err(err) => {
                    handler((table, false));
                    return Err(err).with_context(|| format!("While truncating '{table}' table"));
                }
            }
        }

        Ok(())
    }
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

pub trait TableObject {
    // FIX
    fn table(&self) -> Table;
}
