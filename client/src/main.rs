use std::{
    io::{stdin, stdout, Write},
    process::exit,
};

use common::{send_and_recv, CN_ROUTE, SR_ROUTE};

pub const GAME_KEY: &str = "0 | 1 | 2\n3 | 4 | 5\n6 | 7 | 8";

fn main() {
    let client = Client::new().start();
}

struct Client {
    player: u8,
    route: String,
    board: String,
}
impl Client {
    ///Create a new Client and try to join a game
    fn new() -> Self {
        let r = send_and_recv(String::from("0"), SR_ROUTE, CN_ROUTE);
        if r == "0" {
            println!("Server is full");
            exit(0);
        }
        let res = r.split(',').collect::<Vec<&str>>();
        Client {
            player: res[0].parse::<u8>().unwrap(),
            route: res[1].to_string(),
            board: res[2].to_string(),
        }
    }

    fn start(&mut self) {
        self.display();

        println!("Enter move: ");
        let mut input = String::new();
        self.read(&mut input);
        let r = send_and_recv(input, SR_ROUTE, &self.route);
        //TODO - Validate r
    }

    ///Display which player you are, and the board
    fn display(&self) -> String {
        format!(
            "p: {}|{}\n{}\n{}",
            self.player,
            self.get_icon(),
            GAME_KEY,
            self.board
        )
    }

    ///Get player icon
    fn get_icon(&self) -> &str {
        match self.player {
            1 => "X",
            2 => "O",
            _ => panic!("Player unknown"),
        }
    }

    ///Send the move to the Server
    fn send_move(&self) {
        let r = send_and_recv(
            format!("{},{}", self.player, self.board),
            SR_ROUTE,
            self.route.as_str(),
        );
        if r == "0" {
            println!("server desync, closing");
            exit(0);
        } else {
            println!("move successfully sent");
        }
    }

    ///flush, pass in and read from buffer
    fn read(&self, input: &mut String) {
        stdout().flush().expect("failed to flush");
        stdin().read_line(input).expect("Failed to read");
    }
}
