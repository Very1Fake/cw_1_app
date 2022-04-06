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
    pub const NAME: &'static str = "Warehouse";

    pub const CREATE: &'static str = r#"CREATE TABLE "Warehouse" (
    uuid uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    component uuid NOT NULL REFERENCES "Component" ON DELETE restrict ON UPDATE cascade,
    supplier uuid NOT NULL REFERENCES "Supplier" ON DELETE restrict ON UPDATE cascade,
    price money NOT NULL,
    amount int NOT NULL DEFAULT 0,
    UNIQUE(component, supplier)
);"#;

    pub const DROP: &'static str = r#"DROP TABLE "Warehouse";"#;

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
