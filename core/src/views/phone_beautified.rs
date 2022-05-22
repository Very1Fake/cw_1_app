use mac_address::MacAddress;
use sqlx::{query_as, FromRow};
use uuid::Uuid;

use crate::{
    traits::Recreatable,
    types::{Color, MetaTime},
    PgQueryAs,
};

#[derive(FromRow, Clone, Debug)]
pub struct PhoneBeautified {
    pub uuid: Uuid,
    pub owner: String,
    pub imei: String,
    pub wifi: MacAddress,
    pub bluetooth: MacAddress,
    pub model: String,
    pub color: Color,
    pub meta: MetaTime,
}

impl Recreatable for PhoneBeautified {
    const NAME: &'static str = "PhoneBeautified";

    const CREATE: &'static str = r#"CREATE VIEW "PhoneBeautified" AS
SELECT ph.uuid as uuid,
    CONCAT_WS(' ', p.first_name, p.middle_name, p.last_name) as owner,
    ph.imei as imei,
    ph.wifi as wifi,
    ph.bluetooth as bluetooth,
    pm.name as model,
    ph.color as color,
    ph.meta as meta
FROM "Phone" ph,
    "Person" p,
    "PhoneModel" pm
WHERE ph.person = p.uuid
    AND ph.model = pm.uuid;"#;

    const DROP: &'static str = r#"DROP VIEW "PhoneBeautified";"#;
}

impl PhoneBeautified {
    pub fn get_all() -> PgQueryAs<Self> {
        query_as(r#"SELECT * FROM "PhoneBeautified""#)
    }
}
