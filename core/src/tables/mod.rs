use core::fmt;

use anyhow::{Context, Result};
use sqlx::{postgres::PgQueryResult, query, Error, Executor, PgPool};

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
pub mod service_component;
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
pub use order_component::OrderComponent;
pub use order_service::OrderService;
pub use person::Person;
pub use phone::Phone;
pub use phone_model::PhoneModel;
pub use position::Position;
pub use service::Service;
pub use service_component::ServiceComponent;
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
    ServiceComponent,
    WarehouseSupply,
    OrderService,
    OrderComponent,
}

impl Table {
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
        Self::ServiceComponent,
        Self::WarehouseSupply,
        Self::OrderService,
        Self::OrderComponent,
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
            Table::ServiceComponent => ServiceComponent::NAME,
            Table::WarehouseSupply => WarehouseSupply::NAME,
            Table::OrderService => OrderService::NAME,
            Table::OrderComponent => OrderComponent::NAME,
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
            Table::ServiceComponent => ServiceComponent::CREATE,
            Table::WarehouseSupply => WarehouseSupply::CREATE,
            Table::OrderService => OrderService::CREATE,
            Table::OrderComponent => OrderComponent::CREATE,
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
            Table::ServiceComponent => ServiceComponent::DROP,
            Table::WarehouseSupply => WarehouseSupply::DROP,
            Table::OrderService => OrderService::DROP,
            Table::OrderComponent => OrderComponent::DROP,
        }
    }
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

pub trait TableObject {
    fn table(&self) -> Table;
}

// -------------------------------------------------------------------------------------------------

/// Creates all application tables
pub async fn tb_create_all(
    pool: &PgPool,
    handler: impl Fn((Table, Result<PgQueryResult, Error>)) -> Result<PgQueryResult, Error>,
) -> Result<()> {
    for table in Table::ALL {
        handler((table, pool.execute(query(table.create())).await))
            .with_context(|| format!("While creating '{table}' table"))?;
    }

    Ok(())
}

/// Drops all application tables
pub async fn tb_drop_all(
    pool: &PgPool,
    handler: impl Fn((Table, Result<PgQueryResult, Error>)) -> Result<PgQueryResult, Error>,
) -> Result<()> {
    for table in Table::ALL.into_iter().rev() {
        handler((table, pool.execute(query(table.drop())).await))
            .with_context(|| format!("While dropping '{table}' table"))?;
    }

    Ok(())
}
