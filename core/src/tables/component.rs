use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgArguments, query, query::Query, FromRow, Postgres};
use uuid::Uuid;

use crate::{traits::Insertable, PgQuery};

#[derive(FromRow, Serialize, Deserialize, Clone, Debug)]
pub struct Component {
    pub uuid: Uuid,
    pub name: String,
    /// Foreign key references [`ComponentKind`](`super::component_kind::ComponentKind`)
    pub kind: Uuid,
    /// Foreign key references [`PhoneModel`](`super::phone_model::PhoneModel`)
    pub phone_model: Uuid,
    /// Foreign key references [`Manufacturer`](`super::manufacturer::Manufacturer`)
    pub manufacturer: Uuid,
}

impl Component {
    pub const NAME: &'static str = "Component";

    pub const CREATE: &'static str = r#"CREATE TABLE "Component" (
    uuid uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    name text NOT NULL,
    kind uuid NOT NULL REFERENCES "ComponentKind" ON DELETE restrict ON UPDATE cascade,
    phone_model uuid NOT NULL REFERENCES "PhoneModel" ON DELETE restrict ON UPDATE cascade,
    manufacturer uuid NOT NULL REFERENCES "Manufacturer" ON DELETE restrict ON UPDATE cascade
);"#;

    pub const DROP: &'static str = r#"DROP TABLE "Component";"#;

    pub const SAMPLES: [(&'static str, &'static str, &'static str, &'static str); 30] = [
        ("IPhone 13 Battery", "Battery", "IPhone 13", "FoxCon"),
        (
            "IPhone 13 Pro Battery",
            "Battery",
            "IPhone 13 Pro",
            "FoxCon",
        ),
        ("IPhone XR Battery", "Battery", "IPhone XR", "FoxCon"),
        (
            "IPhone 13 Screen Display",
            "Screen Display",
            "IPhone 13",
            "FoxCon",
        ),
        (
            "IPhone 13 Pro Screen Display",
            "Screen Display",
            "IPhone 13 Pro",
            "FoxCon",
        ),
        (
            "IPhone XR Screen Display",
            "Screen Display",
            "IPhone XR",
            "FoxCon",
        ),
        ("IPhone XR RAM", "RAM", "IPhone XR", "FoxCon"),
        ("IPhone 13 RAM", "RAM", "IPhone 13", "FoxCon"),
        ("IPhone 13 Pro RAM", "RAM", "IPhone 13 Pro", "FoxCon"),
        ("IPhone 13 Memory", "Memory", "IPhone 13", "FoxCon"),
        ("IPhone 13 Pro Memory", "Memory", "IPhone 13 Pro", "FoxCon"),
        ("IPhone XR Memory", "Memory", "IPhone XR", "FoxCon"),
        (
            "IPhone 13 Screen Glass",
            "Screen Glass",
            "IPhone 13",
            "FoxCon",
        ),
        (
            "IPhone 13 Pro Screen Glass",
            "Screen Glass",
            "IPhone 13 Pro",
            "FoxCon",
        ),
        (
            "IPhone XR Screen Glass",
            "Screen Glass",
            "IPhone XR",
            "FoxCon",
        ),
        (
            "Galaxy S21 FE Pro Battery",
            "Battery",
            "Galaxy S21 FE Pro",
            "Samsung",
        ),
        (
            "Galaxy S22 Ultra Battery",
            "Battery",
            "Galaxy S22 Ultra",
            "Samsung",
        ),
        ("Galaxy S22+ Battery", "Battery", "Galaxy S22+", "Samsung"),
        (
            "Galaxy S21 FE Pro Screen Display",
            "Screen Display",
            "Galaxy S21 FE Pro",
            "Samsung",
        ),
        (
            "Galaxy S22 Ultra Screen Display",
            "Screen Display",
            "Galaxy S22 Ultra",
            "Samsung",
        ),
        (
            "Galaxy S22+ Screen Display",
            "Screen Display",
            "Galaxy S22+",
            "Samsung",
        ),
        (
            "Galaxy S21 FE Pro RAM",
            "RAM",
            "Galaxy S21 FE Pro",
            "Samsung",
        ),
        ("Galaxy S22 Ultra RAM", "RAM", "Galaxy S22 Ultra", "Samsung"),
        ("Galaxy S22+ RAM", "RAM", "Galaxy S22+", "Samsung"),
        (
            "Galaxy S21 FE Pro Memory",
            "Memory",
            "Galaxy S21 FE Pro",
            "Samsung",
        ),
        (
            "Galaxy S22 Ultra Memory",
            "Memory",
            "Galaxy S22 Ultra",
            "Samsung",
        ),
        ("Galaxy S22+ Memory", "Memory", "Galaxy S22+", "Samsung"),
        (
            "Galaxy S21 FE Pro Screen Glass",
            "Screen Glass",
            "Galaxy S21 FE Pro",
            "Samsung",
        ),
        (
            "Galaxy S22 Ultra Screen Glass",
            "Screen Glass",
            "Galaxy S22 Ultra",
            "Samsung",
        ),
        (
            "Galaxy S22+ Screen Glass",
            "Screen Glass",
            "Galaxy S22+",
            "Samsung",
        ),
    ];

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
            phone_model: model,
            manufacturer,
        }
    }

    pub fn new_auto(name: String, kind: Uuid, model: Uuid, manufacturer: Uuid) -> Self {
        Self::new(Uuid::new_v4(), name, kind, model, manufacturer)
    }

    pub fn delete_by_uuid(uuid: Uuid) -> PgQuery {
        query(r#"DELETE FROM "Component" WHERE uuid = $1"#).bind(uuid)
    }
}

impl Insertable for Component {
    fn insert(&self) -> Query<'static, Postgres, PgArguments> {
        query(
            r#"INSERT INTO "Component" (uuid, name, kind, phone_model, manufacturer)
VALUES ($1, $2, $3, $4, $5);"#,
        )
        .bind(self.uuid)
        .bind(self.name.clone())
        .bind(self.kind)
        .bind(self.phone_model)
        .bind(self.manufacturer)
    }
}
