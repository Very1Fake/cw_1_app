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
    pub const NAME: &'static str = "Component";

    pub const CREATE: &'static str = r#"CREATE TABLE "Component" (
    uuid uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    name text NOT NULL,
    kind uuid NOT NULL REFERENCES "ComponentKind" ON DELETE restrict ON UPDATE cascade,
    model uuid NOT NULL REFERENCES "PhoneModel" ON DELETE restrict ON UPDATE cascade,
    manufacturer uuid NOT NULL REFERENCES "Manufacturer" ON DELETE restrict ON UPDATE cascade
);"#;

    pub const DROP: &'static str = r#"DROP TABLE "Component";"#;

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
