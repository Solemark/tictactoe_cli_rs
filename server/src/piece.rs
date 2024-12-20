#[derive(PartialEq)]
pub enum Piece {
    ///Cross
    X,
    ///Nought
    O,
    ///None/Empty
    N,
}

impl Piece {
    pub fn get_name(&self) -> &str {
        match self {
            Piece::X => "Crosses",
            Piece::O => "Noughts",
            Piece::N => "",
        }
    }

    pub fn get_icon(&self) -> char {
        match self {
            Piece::X => 'X',
            Piece::O => 'O',
            Piece::N => '_',
        }
    }

    pub fn get_value(&self) -> i8 {
        match self {
            Piece::X => 1,
            Piece::O => -1,
            Piece::N => 0,
        }
    }

    pub fn get_piece(p: &str) -> Piece {
        match p {
            "X" => Piece::X,
            "O" => Piece::O,
            _ => Piece::N,
        }
    }
}
