use sqlx::types::Uuid;

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
    pub const NAME: &'static str = "Person";

    pub const CREATE: &'static str = r#"CREATE TABLE "Person" (
    uuid uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    first_name text NOT NULL,
    middle_name text,
    last_name text NOT NULL,
    email text UNIQUE NOT NULL CHECK (length(email) <= 254),
    phone text UNIQUE NOT NULL CHECK (length(phone) <= 18),
    meta metatime NOT NULL DEFAULT (current_timestamp, current_timestamp)
);"#;

    pub const DROP: &'static str = r#"DROP TABLE "Person";"#;

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
