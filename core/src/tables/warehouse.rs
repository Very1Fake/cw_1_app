use uuid::Uuid;

#[derive(Debug)]
pub struct Warehouse {
    pub uuid: Uuid,
    /// Foreign key references [`Component`](`super::component::Component`)
    pub component: Uuid,
    /// Foreign key references [`Supplier`](`super::supplier::Supplier`)
    pub supplier: Uuid,
    pub price: f64,
    pub amount: i32,
}

impl Warehouse {
    pub const fn new(uuid: Uuid, component: Uuid, supplier: Uuid, price: f64, amount: i32) -> Self {
        Self {
            uuid,
            component,
            supplier,
            price,
            amount,
        }
    }
}
