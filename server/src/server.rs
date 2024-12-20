use {
    crate::piece::Piece,
    crate::tictactoe::TicTacToe,
    common::{handle_response, send_message, BIND_ERR, CN_ROUTE, P1_ROUTE, P2_ROUTE, SR_ROUTE},
    std::net::TcpListener,
};

pub struct Server {
    ///current board state
    board: TicTacToe,
    ///player count
    p_count: u8,
    ///player turn
    p_turn: u8,
    ///turn count
    t_count: u8,
}
impl Server {
    ///Create a new Server
    pub fn new() -> Self {
        Server {
            board: TicTacToe::new(),
            p_count: 0,
            p_turn: 1,
            t_count: 0,
        }
    }

    ///Start the Server
    pub fn start(&mut self) {
        for stream in TcpListener::bind(SR_ROUTE).expect(BIND_ERR).incoming() {
            let msg = handle_response(stream.unwrap());
            if msg == "0" {
                self.handle_new_game();
            } else {
                self.handle_existing_game(msg);
            }
        }
    }

    ///Assign new player if >2 players
    fn handle_new_game(&mut self) {
        if self.p_count < 2 {
            self.assign_player();
            self.incr_player();
        }
    }

    ///Assign player
    fn assign_player(&self) {
        let msg = match self.p_count {
            0 => format!("1,{},{}", P1_ROUTE, self.board.draw()),
            1 => format!("2,{},{}", P2_ROUTE, self.board.draw()),
            _ => format!("0"),
        };
        send_message(msg, CN_ROUTE);
    }

    ///Incriment player count
    fn incr_player(&mut self) {
        self.p_count = match self.p_count {
            0 => 1,
            1 => 2,
            _ => 3,
        };
    }

    ///Handle in progress game
    fn handle_existing_game(&mut self, msg: String) {
        println!("message recieved: {}", msg);
        let m = msg.split(',').collect::<Vec<&str>>();
        if self.check_player_turn(&m) {
            self.t_count += 1;
            self.update_board(m);
            self.update_players();
        }
    }

    //Check its players turn
    fn check_player_turn(&self, m: &Vec<&str>) -> bool {
        let p = m[0].trim().parse::<u8>().unwrap_or_default();
        p == self.p_turn
    }

    ///Update the board
    fn update_board(&mut self, m: Vec<&str>) {
        let b = String::from(m[1]).replace('\n', "");
        let b = b.split('|').collect::<Vec<&str>>();
        for i in 0..=8 {
            self.board.board[i] = Piece::get_piece(b[i]);
        }
    }

    //Send board to other player
    fn update_players(&mut self) {
        let m = format!("{},{}", self.t_count, self.board.draw());
        if self.p_turn == 1 {
            self.p_turn = 2;
            send_message(m, P2_ROUTE);
        } else {
            self.p_turn = 1;
            send_message(m, P1_ROUTE);
        }
    }
}
