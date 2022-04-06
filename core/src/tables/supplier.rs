use isocountry::CountryCode;
use uuid::Uuid;

#[derive(Debug)]
pub struct Supplier {
    pub uuid: Uuid,
    pub name: String,
    pub iban: String,
    pub swift: String,
    pub address: String,
    pub country: CountryCode,
}

impl Supplier {
    pub const NAME: &'static str = "Supplier";

    pub const CREATE: &'static str = r#"CREATE TABLE "Supplier" (
    uuid uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    name text NOT NULL,
    iban text NOT NULL UNIQUE CHECK (length(iban) <= 32),
    swift text NOT NULL CHECK (length(swift) <= 11),
    address text NOT NULL,
    country char(2) NOT NULL,
    details json
);"#;

    pub const DROP: &'static str = r#"DROP TABLE "Supplier";"#;

    pub const fn new(
        uuid: Uuid,
        name: String,
        iban: String,
        swift: String,
        address: String,
        country: CountryCode,
    ) -> Self {
        Self {
            uuid,
            name,
            iban,
            swift,
            address,
            country,
        }
    }
}
