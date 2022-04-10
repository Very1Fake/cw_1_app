use sqlx::{postgres::PgQueryResult, query, Error, PgPool};
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
    pub const NAME: &'static str = "Person";

    pub const CREATE: &'static str = r#"CREATE TABLE "Person" (
    uuid uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    first_name text NOT NULL,
    middle_name text,
    last_name text NOT NULL,
    email text UNIQUE NOT NULL CHECK (length(email) <= 254),
    phone text UNIQUE NOT NULL CHECK (length(phone) <= 18),
    meta metatime NOT NULL DEFAULT (now(), now())
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

    pub fn new_auto(
        f_name: String,
        m_name: Option<String>,
        l_name: String,
        email: String,
        phone: String,
    ) -> Self {
        Self::new(
            Uuid::new_v4(),
            f_name,
            m_name,
            l_name,
            email,
            phone,
            MetaTime::default(),
        )
    }

    pub async fn insert(&self, pool: &PgPool) -> Result<PgQueryResult, Error> {
        query(
            r#"INSERT INTO "Person"
(uuid, first_name, middle_name, last_name, email, phone) 
VALUES ($1, $2, $3, $4, $5, $6);"#,
        )
        .bind(self.uuid)
        .bind(self.first_name.clone())
        .bind(self.middle_name.clone())
        .bind(self.last_name.clone())
        .bind(self.email.clone())
        .bind(self.phone.clone())
        .execute(pool)
        .await
    }
}
