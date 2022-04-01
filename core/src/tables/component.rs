use uuid::Uuid;

pub struct Component {
    pub uuid: Uuid,
    pub name: String,
    /// Foreign key references [`ComponentKind`](`super::component_kind::ComponentKind`)
    pub kind: Uuid,
    /// Foreign key references [`PhoneModel`](`super::phone_model::PhoneModel`)
    pub model: Uuid,
    /// Foreign key references [`Manufacturer`](`super::manufacturer::Manufacturer`)
    pub manufacturer: Uuid,
}

impl Component {
    pub const fn new(
        uuid: Uuid,
        name: String,
        kind: Uuid,
        model: Uuid,
        manufacturer: Uuid,
    ) -> Self {
        Self {
            uuid,
            name,
            kind,
            model,
            manufacturer,
        }
    }
}
