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
