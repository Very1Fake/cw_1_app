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
