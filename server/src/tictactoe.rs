use crate::{
    consts::{ALL_CHECK, NEW_BOARD},
    piece::Piece,
};

pub struct TicTacToe {
    pub board: [Piece; 9],
}
impl TicTacToe {
    /** Create a new game board */
    pub fn new() -> TicTacToe {
        TicTacToe { board: NEW_BOARD }
    }

    /**draw the board*/
    pub fn draw(&self) -> String {
        let (mut output, mut c) = (String::new(), 0);
        for m in &self.board {
            output.push('|');
            output.push(m.get_icon());
            c += 1;
            if c % 3 == 0 {
                c = 0;
                output.push('|');
                output.push('\n');
            }
        }
        output
    }

    /**Check if input is a board position and if space is available*/
    pub fn check_legal_move(&self, pos: usize) -> bool {
        if pos > 8 {
            return false;
        }
        if self.board[pos] != Piece::N {
            return false;
        }
        true
    }

    /**check all possible "win states" on the board*/
    pub fn check_board(&self) -> String {
        for i in ALL_CHECK {
            let val = self.check_cells(&self.board[i[0]], &self.board[i[1]], &self.board[i[2]]);
            if val != Piece::N {
                return format!("{} wins", val.get_name());
            }
        }
        String::new()
    }

    /**check if the provided board positions result in a win*/
    fn check_cells(&self, a: &Piece, b: &Piece, c: &Piece) -> Piece {
        match a.get_value() + b.get_value() + c.get_value() {
            3 => Piece::X,
            -3 => Piece::O,
            _ => Piece::N,
        }
    }
}
