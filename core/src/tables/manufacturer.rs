use isocountry::CountryCode;
use uuid::Uuid;

#[derive(Debug)]
pub struct Manufacturer {
    pub uuid: Uuid,
    pub name: String,
    pub country: CountryCode,
}

impl Manufacturer {
    pub const NAME: &'static str = "Manufacturer";

    pub const CREATE: &'static str = r#"CREATE TABLE "Manufacturer" (
    uuid uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    name text,
    country char(2) NOT NULL
);"#;

    pub const DROP: &'static str = r#"DROP TABLE "Manufacturer";"#;

    pub const fn new(uuid: Uuid, name: String, country: CountryCode) -> Self {
        Self {
            uuid,
            name,
            country,
        }
    }
}
