#[derive(Debug)]
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
    pub const NAME: &'static str = "color";

    pub const CREATE: &'static str = "CREATE TYPE color AS ENUM (
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

    pub const DROP: &'static str = "DROP TYPE color;";
}
