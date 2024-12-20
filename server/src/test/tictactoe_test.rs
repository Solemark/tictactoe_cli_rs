#[cfg(test)]
mod tests {
    use crate::{piece::Piece, tictactoe::TicTacToe};

    #[test]
    fn test_draw_board() {
        let exp = String::from("|O|O|O|\n|X|_|X|\n|X|_|X|\n");
        let res = TicTacToe {
            board: [
                Piece::O,
                Piece::O,
                Piece::O,
                Piece::X,
                Piece::N,
                Piece::X,
                Piece::X,
                Piece::N,
                Piece::X,
            ],
        }
        .draw();
        assert_eq!(exp, res);
    }
}
