use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgArguments, query, query::Query, Postgres};
use uuid::Uuid;

use crate::traits::Insertable;

#[derive(Serialize, Deserialize, Debug)]
pub struct PhoneModel {
    pub uuid: Uuid,
    pub name: String,
    pub description: Option<String>,
    /// Foreign key references [`Manufacturer`](super::manufacturer::Manufacturer)
    pub manufacturer: Uuid,
}

impl PhoneModel {
    pub const NAME: &'static str = "PhoneModel";

    pub const CREATE: &'static str = r#"CREATE TABLE "PhoneModel" (
    uuid uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    name text NOT NULL UNIQUE,
    description text,
    manufacturer uuid NOT NULL REFERENCES "Manufacturer" ON DELETE restrict ON UPDATE cascade
);"#;

    pub const DROP: &'static str = r#"DROP TABLE "PhoneModel";"#;

    pub const SAMPLES: [(&'static str, &'static str, &'static str, f64); 6] = [
        ("IPhone 13", "A dramatically more powerful camera system. A display so responsive, every interaction feels new again. The world's fastest smartphone chip. Exceptional durability. And a huge leap in battery life. Let's Pro.", "Apple", 1.45),
        ("IPhone 13 Pro", "", "Apple", 1.5),
        ("IPhone XR", "", "Apple", 1.25),
        ("Galaxy S21 FE Pro", "Get more out of the activities you heart most with Galaxy S21 FE 5G. We took all your favorites and built the ultimate fan-inspired phone jam-packed with features to fuel your passions. Whether you're a gaming guru or social media star, this crowd pleaser has the style, power and pro-grade camera to unleash epic in the everyday.", "Samsung", 1.3),
        ("Galaxy S22 Ultra", "", "Samsung", 1.3),
        ("Galaxy S22+", "", "Samsung", 1.25),
    ];

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

    pub fn new_auto(name: String, description: Option<String>, manufacturer: Uuid) -> Self {
        Self::new(Uuid::new_v4(), name, description, manufacturer)
    }
}

impl Insertable for PhoneModel {
    fn insert(&self) -> Query<'static, Postgres, PgArguments> {
        query(
            r#"INSERT INTO "PhoneModel" (uuid, name, description, manufacturer) 
    VALUES ($1, $2, $3, $4);"#,
        )
        .bind(self.uuid)
        .bind(self.name.clone())
        .bind(self.description.clone())
        .bind(self.manufacturer)
    }
}
