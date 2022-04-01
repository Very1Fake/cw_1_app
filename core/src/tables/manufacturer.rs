use isocountry::CountryCode;
use uuid::Uuid;

#[derive(Debug)]
pub struct Manufacturer {
    pub uuid: Uuid,
    pub name: String,
    pub country: CountryCode,
}

impl Manufacturer {
    pub const fn new(uuid: Uuid, name: String, country: CountryCode) -> Self {
        Self {
            uuid,
            name,
            country,
        }
    }
}
