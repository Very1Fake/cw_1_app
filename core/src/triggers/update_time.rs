use crate::{
    tables::Table,
    traits::{MultipleTables, Recreatable},
};

pub struct UpdateTime;

impl Recreatable for UpdateTime {
    const NAME: &'static str = "update_time";
    const CREATE: &'static str = r#"CREATE TRIGGER update_time AFTER
UPDATE ON "$1" FOR EACH ROW
    WHEN (pg_trigger_depth() = 0) EXECUTE FUNCTION update_time_func();"#;
    const DROP: &'static str = r#"DROP TRIGGER update_time ON "$1";"#;
}

impl MultipleTables<12> for UpdateTime {
    const TABLES: [Table; 12] = [
        Table::Person,
        Table::Supplier,
        Table::Position,
        Table::Service,
        Table::LaborContract,
        Table::Phone,
        Table::SupplyContract,
        Table::Supply,
        Table::Account,
        Table::Warehouse,
        Table::Order,
        Table::ServicePhoneModel,
    ];
}
