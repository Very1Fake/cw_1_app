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

    pub fn as_str(&self) -> &str {
        match self {
            Self::Black => "Black",
            Self::DarkGray => "Dark Grey",
            Self::Gray => "Gray",
            Self::LightGray => "Light Gray",
            Self::White => "White",
            Self::Brown => "Brown",
            Self::DarkRed => "Dark Red",
            Self::Red => "Red",
            Self::LightRed => "Light Red",
            Self::Yellow => "Yellow",
            Self::LightYellow => "Light Yellow",
            Self::Khaki => "Khaki",
            Self::DarkGreen => "Dark Green",
            Self::Green => "Green",
            Self::LightGreen => "Light Green",
            Self::DarkBlue => "Dark Blue",
            Self::Blue => "Blue",
            Self::LightBlue => "Light Blue",
            Self::Gold => "Gold",
        }
    }
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
