mod consts;
mod piece;
mod tictactoe;

use {
    common::{handle_response, send_message, BIND_ERR, CN_ROUTE, P1_ROUTE, P2_ROUTE, SR_ROUTE},
    std::net::TcpListener,
    tictactoe::TicTacToe,
};

fn main() {
    Server::new().start();
}

struct Server {
    ///current board state
    board: TicTacToe,
    ///player count
    p_count: u8,
}
impl Server {
    ///Create a new Server
    fn new() -> Self {
        Server {
            board: TicTacToe::new(),
            p_count: 0,
        }
    }

    ///Start the Server
    fn start(&mut self) {
        for stream in TcpListener::bind(SR_ROUTE).expect(BIND_ERR).incoming() {
            let msg = handle_response(stream.unwrap());
            if msg == "0" {
                self.handle_new_player();
            } else {
                self.handle_existing_player(msg);
            }
        }
    }

    fn handle_new_player(&mut self) {
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

    fn handle_existing_player(&mut self, msg: String) {
        println!("message recieved: {}", msg);
        // TODO - Check its players turn
        // TODO - Update the board
        // TODO - Send board to other player
    }
}
