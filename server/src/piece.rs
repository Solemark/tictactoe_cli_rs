#[derive(PartialEq)]
pub enum Piece {
    X, // Cross
    O, // Nought
    N, // None
}

impl Piece {
    pub fn get_name(&self) -> &str {
        match self {
            Piece::X => "Crosses",
            Piece::O => "Noughts",
            Piece::N => "",
        }
    }

    pub fn get_icon(&self) -> &str {
        match self {
            Piece::X => "|X|",
            Piece::O => "|O|",
            Piece::N => "|_|",
        }
    }

    pub fn get_value(&self) -> i8 {
        match self {
            Piece::X => 1,
            Piece::O => -1,
            Piece::N => 0,
        }
    }
}
