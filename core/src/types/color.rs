use serde::{Deserialize, Serialize};

use crate::traits::Recreatable;

#[derive(Serialize, Deserialize, sqlx::Type, Clone, Copy, Debug)]
#[sqlx(type_name = "color", rename_all = "PascalCase")]
pub enum Color {
    Black,
    DarkGray,
    Gray,
    LightGray,
    White,
    Brown,
    DarkRed,
    Red,
    LightRed,
    Yellow,
    LightYellow,
    Khaki,
    DarkGreen,
    Green,
    LightGreen,
    DarkBlue,
    Blue,
    LightBlue,
    Gold,
}

impl Color {
    pub const ALL: [Self; 19] = [
        Self::Black,
        Self::DarkGray,
        Self::Gray,
        Self::LightGray,
        Self::White,
        Self::Brown,
        Self::DarkRed,
        Self::Red,
        Self::LightRed,
        Self::Yellow,
        Self::LightYellow,
        Self::Khaki,
        Self::DarkGreen,
        Self::Green,
        Self::LightGreen,
        Self::DarkBlue,
        Self::Blue,
        Self::LightBlue,
        Self::Gold,
    ];
}

impl Recreatable for Color {
    const NAME: &'static str = "color";

    const CREATE: &'static str = "CREATE TYPE color AS ENUM (
    'Black',
    'DarkGray',
    'Gray',
    'LightGray',
    'White',
    'Brown',
    'DarkRed',
    'Red',
    'LightRed',
    'Yellow',
    'LightYellow',
    'Khaki',
    'DarkGreen',
    'Green',
    'LightGreen',
    'DarkBlue',
    'Blue',
    'LightBlue',
    'Gold'
);";

    const DROP: &'static str = "DROP TYPE color;";
}
