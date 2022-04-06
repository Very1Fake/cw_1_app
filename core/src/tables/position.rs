use crate::types::metatime::MetaTime;

#[derive(Debug)]
pub struct Position {
    pub id: i32,
    pub name: String,
    pub details: Option<String>,
    pub salary: f64,
    pub meta: MetaTime,
}

impl Position {
    pub const NAME: &'static str = "Position";

    pub const CREATE: &'static str = r#"CREATE TABLE "Position" (
    id int PRIMARY KEY,
    name text NOT NULL,
    details text,
    salary money NOT NULL,
    meta metatime NOT NULL DEFAULT (current_timestamp, current_timestamp)
);"#;

    pub const DROP: &'static str = r#"
    DROP TABLE "Position";
    "#;

    pub const fn new(
        id: i32,
        name: String,
        details: Option<String>,
        salary: f64,
        meta: MetaTime,
    ) -> Self {
        Self {
            id,
            name,
            details,
            salary,
            meta,
        }
    }
}
