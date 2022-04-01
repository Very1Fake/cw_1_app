use uuid::Uuid;

use crate::types::metatime::MetaTime;

#[derive(Debug)]
pub struct Person {
    pub uuid: Uuid,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub email: String,
    pub phone: String,
    pub meta: MetaTime,
}

impl Person {
    pub const fn new(
        uuid: Uuid,
        f_name: String,
        m_name: Option<String>,
        l_name: String,
        email: String,
        phone: String,
        meta: MetaTime,
    ) -> Self {
        Self {
            uuid,
            first_name: f_name,
            middle_name: m_name,
            last_name: l_name,
            email,
            phone,
            meta,
        }
    }
}
