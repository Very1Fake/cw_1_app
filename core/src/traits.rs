use sqlx::{postgres::PgArguments, query::Query, Postgres};

pub trait Recreatable {
    const NAME: &'static str;
    const CREATE: &'static str;
    const DROP: &'static str;
}

pub trait Insertable {
    fn insert(&self) -> Query<'static, Postgres, PgArguments>;
}
