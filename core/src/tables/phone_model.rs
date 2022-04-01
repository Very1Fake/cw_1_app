use uuid::Uuid;

#[derive(Debug)]
pub struct PhoneModel {
    pub uuid: Uuid,
    pub name: String,
    pub description: Option<String>,
    /// Foreign key references [`Manufacturer`](super::manufacturer::Manufacturer)
    pub manufacturer: Uuid,
}

impl PhoneModel {
    pub const fn new(
        uuid: Uuid,
        name: String,
        description: Option<String>,
        manufacturer: Uuid,
    ) -> Self {
        Self {
            uuid,
            name,
            description,
            manufacturer,
        }
    }
}
